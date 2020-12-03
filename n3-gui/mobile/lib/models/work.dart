import 'package:flutter/material.dart';
import 'package:n3_mobile/models/base.dart';
import 'package:n3_mobile/models/net.dart';

class Work extends DBTable {
  // 128-bits unsigned Integer
  final BigInt id;
  final Command command;
  final String exec;
  final Map<String, String> variables;

  final WorkStatus status;

  const Work({
    @required this.id,
    @required this.command,
    @required this.exec,
    @required this.variables,
    @required this.status,
  });

  static Work fromJson(Map<String, dynamic> source) {
    return Work(
      // String -> BigInt
      id: BigInt.parse(source['id']),
      command: CommandToString.commandFromString(source['command']),
      exec: source['exec'],
      variables: Map<String, String>.from(source['variables']),
      status: WorkStatus.fromJson(source['status']),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      // BigInt -> String
      'id': this.id.toString(),
      'command': this.command.commandToString(),
      'exec': this.exec,
      'variables': this.variables,
      'status': this.status.toJson(),
    };
  }

  static Future<Work> get(BuildContext context, BigInt id) async {
    return await Net().getOne(
      context: context,
      url: 'work/$id',
      generator: fromJson,
      onConnectionFailure: () async {},
      onInternalFailure: () async {},
    );
  }

  static Future<List<Work>> getList(BuildContext context) async {
    return await Net().getList(
      context: context,
      url: 'work',
      generator: fromJson,
      onConnectionFailure: () async {},
      onInternalFailure: () async {},
    );
  }
}

class WorkStatus {
  final bool isRunning;
  final String errorMsg;

  // Local Time (not UTC)
  final DateTime dateBegin;
  final DateTime dateEnd;

  const WorkStatus({
    @required this.isRunning,
    this.errorMsg,
    @required this.dateBegin,
    this.dateEnd,
  });

  static WorkStatus fromJson(Map<String, dynamic> source) {
    return WorkStatus(
      isRunning: source['is_running'],
      errorMsg: source['error_msg'],
      // UTC -> Local
      dateBegin: _tryParseDateTime(source['date_begin']),
      dateEnd: _tryParseDateTime(source['date_end']),
    );
  }

  static DateTime _tryParseDateTime(dynamic dateTime) {
    if (dateTime != null) {
      return DateTime.tryParse(dateTime)?.toLocal();
    }
    return null;
  }

  Map<String, dynamic> toJson() {
    return {
      'is_running': this.isRunning,
      'error_msg': this.errorMsg,
      // Local -> UTC
      'date_begin': this.dateBegin?.toUtc().toString(),
      'date_end': this.dateEnd?.toUtc().toString(),
    };
  }
}

enum Command {
  Unknown,
  Train,
  Eval,
}

extension CommandToString on Command {
  static Command commandFromString(String command) {
    switch (command) {
      case 'train':
        return Command.Train;
      case 'eval':
        return Command.Eval;
      case '':
      default:
        return Command.Unknown;
    }
  }

  String commandToString() {
    switch (this) {
      case Command.Train:
        return 'train';
      case Command.Eval:
        return 'eval';
      case Command.Unknown:
      default:
        return '';
    }
  }
}
