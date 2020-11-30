import 'package:flutter/material.dart';
import 'package:n3_mobile/pages/intro.dart';
import 'package:n3_mobile/pages/panel/main.dart';

const String initialRoute = '/';

Map<String, WidgetBuilder> routes() {
  return {
    '/': (context) => IntroPage(),
    '/panel': (context) => PanelMainPage(),
  };
}
