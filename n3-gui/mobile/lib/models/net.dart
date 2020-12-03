import 'dart:convert';
import 'dart:io';

import 'package:cookie_jar/cookie_jar.dart';
import 'package:dio/dio.dart';
import 'package:dio_cookie_manager/dio_cookie_manager.dart';
import 'package:flutter/material.dart';
import 'package:n3_mobile/models/base.dart';
import 'package:n3_mobile/res/message/net.dart';
import 'package:n3_mobile/widgets/dialog/simple.dart';
import 'package:path_provider/path_provider.dart';
import 'package:flutter/foundation.dart' show kIsWeb;

typedef List<dynamic> PreprocessFunction(dynamic data);
typedef T GeneratorFunction<T>(Map<String, dynamic> json);
typedef Future<Response> RequestFunction(Dio dio, String url, {String data});
typedef Future<void> FallbackFunction();

class Net {
  // TODO: to be implemented
  static const String host = 'localhost:8000';
  static const String apiUrl = '';

  static final Net _instance = Net._internal();
  factory Net() => _instance;
  Net._internal();

  Future<Dio> dio() async {
    final dio = Dio();

    if (!kIsWeb) {
      final appDocDir = await getApplicationDocumentsDirectory();
      final appDocPath = appDocDir.path;

      final cookieJar = PersistCookieJar(dir: appDocPath + '/.cookies/');
      final cookieManager = CookieManager(cookieJar);

      dio.interceptors.add(cookieManager);
    }

    dio.options.contentType = 'application/json';
    dio.options.headers[HttpHeaders.acceptHeader] = 'application/json';
    dio.options.headers[HttpHeaders.acceptCharsetHeader] = 'utf-8';
    dio.options.responseType = ResponseType.bytes;
    dio.options.validateStatus = (status) => status < 600;
    return dio;
  }

  Future<T> _error<T>({
    @required BuildContext context,
    @required FallbackFunction f,
    @required String messageLog,
    @required String messageDialog,
  }) async {
    if (f != null) {
      print('[NET] $messageLog');
      await f();
    } else {
      await showMessageDialog(
        context: context,
        content: messageDialog,
        onConfirm: exitApp,
      );
    }
    return null;
  }

  Future<T> _errorConnectionFailure<T>(
    BuildContext context,
    DioError e,
    FallbackFunction f,
  ) async {
    return _error(
      context: context,
      f: f,
      messageLog: 'Connection Error: $e',
      messageDialog: NetMessage.connectionFailure,
    );
  }

  Future<T> _errorInternalFailure<T>(
    BuildContext context,
    Response response,
    FallbackFunction f,
  ) async {
    return _error(
      context: context,
      f: f,
      messageLog: 'Internal Error: ${response.data}',
      messageDialog: NetMessage.internalFailure,
    );
  }

  Future<dynamic> _request({
    @required BuildContext context,
    @required String url,
    Map<String, dynamic> queries,
    @required RequestFunction f,
    FallbackFunction onConnectionFailure,
    FallbackFunction onInternalFailure,
  }) async {
    final uri = Uri.http(host, '$apiUrl/$url/');

    try {
      final response = await f(
        await this.dio(),
        uri.toString(),
        data: json.encode(queries ?? {}),
      );

      // internal failure
      if (![200, 201].contains(response.statusCode)) {
        return await _errorInternalFailure(
          context,
          response,
          onInternalFailure,
        );
      }

      // internal failure - json
      final data = jsonDecode(utf8.decode(response.data));
      if (!data['success']) {
        return await _errorInternalFailure(
          context,
          response,
          onInternalFailure,
        );
      }

      return data['data'];
    } on DioError catch (e) {
      return await _errorConnectionFailure(
        context,
        e,
        onInternalFailure,
      );
    }
  }

  Future<dynamic> _get({
    @required BuildContext context,
    @required String url,
    Map<String, String> queries,
    FallbackFunction onConnectionFailure,
    FallbackFunction onInternalFailure,
  }) async {
    final uri = Uri.http(host, '$apiUrl/$url/', queries).toString();
    return _request(
      f: (dio, _, {data}) => dio.get(uri),
      context: context,
      url: url,
      queries: queries,
      onConnectionFailure: onConnectionFailure,
      onInternalFailure: onInternalFailure,
    );
  }

  Future<T> getOne<T extends DBTable>({
    @required BuildContext context,
    @required String url,
    Map<String, String> queries,
    @required GeneratorFunction<T> generator,
    FallbackFunction onConnectionFailure,
    FallbackFunction onInternalFailure,
  }) async {
    var data = await _get(
      context: context,
      url: url,
      queries: queries,
      onConnectionFailure: onConnectionFailure,
      onInternalFailure: onInternalFailure,
    );

    if (data != null) {
      return generator(data);
    } else {
      return null;
    }
  }

  Future<List<T>> getList<T extends DBTable>({
    @required BuildContext context,
    @required String url,
    Map<String, String> queries,
    PreprocessFunction preprocess,
    @required GeneratorFunction<T> generator,
    FallbackFunction onConnectionFailure,
    FallbackFunction onInternalFailure,
  }) async {
    var data = await _get(
      context: context,
      url: url,
      queries: queries,
      onConnectionFailure: onConnectionFailure,
      onInternalFailure: onInternalFailure,
    );

    if (data != null) {
      final List<dynamic> dataPreprocessed =
          preprocess != null ? preprocess(data) : List<dynamic>.from(data);
      return dataPreprocessed.map((e) => generator(e)).toList();
    } else {
      return null;
    }
  }

  Future<Map<K, T>> getDict<T extends DBTable<K>, K>({
    @required BuildContext context,
    @required String url,
    Map<String, String> queries,
    @required GeneratorFunction<T> generator,
    FallbackFunction onConnectionFailure,
    FallbackFunction onInternalFailure,
  }) async {
    final List<dynamic> data = await _get(
      context: context,
      url: url,
      queries: queries,
      onConnectionFailure: onConnectionFailure,
      onInternalFailure: onInternalFailure,
    );

    if (data != null) {
      final List<T> contents = data.map((e) => generator(e)).toList();
      return Map.fromIterable(contents, key: (e) => e.id, value: (e) => e);
    } else {
      return null;
    }
  }

  Future<dynamic> _post({
    @required BuildContext context,
    @required String url,
    @required Map<String, dynamic> queries,
    List<File> files,
    FallbackFunction onConnectionFailure,
    FallbackFunction onInternalFailure,
  }) async {
    return _request(
      f: (dio, url, {data}) async {
        // add files
        if (files != null) {
          dio.options.contentType = 'multipart/form-data';

          // add a file
          final fileList = await Future.wait(
            files.map((file) {
              final filename = file.path.split('/').last;
              return MultipartFile.fromFile(file.path, filename: filename);
            }),
          );
          final formData = fileList.asMap().map((key, value) =>
              MapEntry('__file_${key.toString()}', value as dynamic));

          // add data
          for (final query in queries.entries) {
            formData[query.key] = query.value.toString();
          }

          return dio.post(
            url,
            data: FormData.fromMap(formData),
          );
        }
        // send request
        return dio.post(url, data: data);
      },
      context: context,
      url: url,
      queries: queries,
      onConnectionFailure: onConnectionFailure,
      onInternalFailure: onInternalFailure,
    );
  }

  Future<T> postOne<T extends DBTable>({
    @required BuildContext context,
    @required String url,
    @required Map<String, dynamic> queries,
    @required GeneratorFunction<T> generator,
    FallbackFunction onConnectionFailure,
    FallbackFunction onInternalFailure,
  }) async {
    final Map<String, dynamic> data = await _post(
      context: context,
      url: url,
      queries: queries,
      onConnectionFailure: onConnectionFailure,
      onInternalFailure: onInternalFailure,
    );

    if (data != null) {
      return generator(data);
    } else {
      return null;
    }
  }

  Future<bool> createOne<T extends DBTable>({
    @required BuildContext context,
    @required String url,
    @required T object,
    List<File> files,
  }) async {
    return createOneWithQuery(
      context: context,
      url: url,
      queries: object.toJson(),
      files: files,
    );
  }

  Future<bool> createOneWithQuery({
    @required BuildContext context,
    @required String url,
    @required Map<String, dynamic> queries,
    List<File> files,
    FallbackFunction onInternalFailure,
  }) async {
    final Map<String, dynamic> data = await _post(
      context: context,
      url: url,
      queries: queries,
      files: files,
      onConnectionFailure: () async => showMessageDialog(
        context: context,
        content: NetMessage.connectionFailure,
      ),
      onInternalFailure: onInternalFailure ?? () async => {},
    );

    if (data != null) {
      return true;
    } else {
      return false;
    }
  }

  Future<dynamic> _update({
    @required BuildContext context,
    @required String url,
    @required Map<String, dynamic> queries,
    FallbackFunction onConnectionFailure,
    FallbackFunction onInternalFailure,
  }) async {
    return _request(
      f: (dio, url, {data}) => dio.patch(url, data: data),
      context: context,
      url: url,
      queries: queries,
      onConnectionFailure: onConnectionFailure,
      onInternalFailure: onInternalFailure,
    );
  }

  Future<bool> update({
    @required BuildContext context,
    @required String url,
    @required Map<String, dynamic> queries,
  }) async {
    final Map<String, dynamic> data = await _update(
      context: context,
      url: url,
      queries: queries,
      onConnectionFailure: () async => showMessageDialog(
        context: context,
        content: NetMessage.connectionFailure,
      ),
      onInternalFailure: () async => {},
    );

    if (data != null) {
      return true;
    } else {
      return false;
    }
  }
}
