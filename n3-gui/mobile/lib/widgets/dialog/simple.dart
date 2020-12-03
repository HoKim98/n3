import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

/// 앱을 종료합니다.
void exitApp() {
  SystemChannels.platform.invokeMethod('SystemNavigator.pop');
}

/// 일반적인 메세지 알림창을 날립니다.
Future<void> showMessageDialog({
  BuildContext context,
  String title = '알림',
  String content,
  void Function() onConfirm,
  bool barrierDismissible = false,
}) async {
  return showDialog(
    context: context,
    barrierDismissible: barrierDismissible,
    builder: (BuildContext context) {
      return WillPopScope(
        child: AlertDialog(
          title: Text(title),
          content: Text(content),
          actions: <Widget>[
            TextButton(
              child: Text('확인'),
              onPressed: () {
                Navigator.of(context).pop();
                if (onConfirm != null) onConfirm();
              },
            ),
          ],
        ),
        onWillPop: () async {
          if (barrierDismissible && onConfirm != null) onConfirm();
          return true;
        },
      );
    },
  );
}

/// 네/아니오 선택이 가능한 알림창을 날립니다.
Future<void> showYesNoDialog({
  BuildContext context,
  String title = '알림',
  String content,
  void Function() onAccept,
  void Function() onDeny,
}) async {
  return showDialog(
    context: context,
    barrierDismissible: false, // user must tap button!
    builder: (BuildContext context) {
      return WillPopScope(
        child: AlertDialog(
          title: Text(title),
          content: Text(content),
          actions: <Widget>[
            TextButton(
              child: Text('네'),
              onPressed: () {
                Navigator.of(context).pop();
                if (onAccept != null) onAccept();
              },
            ),
            TextButton(
              child: Text('아니오'),
              onPressed: () {
                Navigator.of(context).pop();
                if (onDeny != null) onDeny();
              },
            ),
          ],
        ),
        onWillPop: () async {
          if (onDeny != null) onDeny();
          return true;
        },
      );
    },
  );
}
