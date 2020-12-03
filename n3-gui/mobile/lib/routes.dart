import 'package:flutter/material.dart';
import 'package:n3_mobile/pages/intro.dart';
import 'package:n3_mobile/pages/panel/main.dart';
import 'package:n3_mobile/pages/panel/works/detail.dart';

const String initialRoute = '/';

Map<String, WidgetBuilder> routes = {
  '/': (context) => IntroPage(),
  '/panel': (context) => PanelMainPage(),
  '/panel/work': (context) => WorkDetailPage(
        ModalRoute.of(context).settings.arguments,
      ),
};
