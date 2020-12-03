import 'package:flutter/material.dart';
import 'package:n3_mobile/widgets/dialog/simple.dart';

/// 작성 중 뒤로 가려고 하는 경우, 정말로 뒤로 갈 것인지 물어봅니다.
Future<bool> onExitForm(BuildContext context) async {
  var result = false;
  await showYesNoDialog(
    context: context,
    content: '작성 중 취소시 저장이 되지 않습니다. 정말 취소하시겠습니까?',
    onAccept: () => result = true,
  );
  return result;
}
