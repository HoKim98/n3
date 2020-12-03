import 'package:flutter/material.dart';
import 'package:n3_mobile/widgets/dialog/simple.dart';

String tryGetString(
  BuildContext context,
  TextEditingController controller,
  FocusNode focusNode,
  String label,
) {
  final text = controller.text;
  if (text.isEmpty) {
    showMessageDialog(
      context: context,
      content: '$label을(를) 입력해주세요.',
      onConfirm: () => focusNode.requestFocus(),
    );
    return null;
  }
  return text;
}
