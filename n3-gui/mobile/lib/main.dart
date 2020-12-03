import 'package:flutter/material.dart';
import 'package:n3_mobile/routes.dart';

void main() {
  runApp(App());
}

class App extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'n3',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      initialRoute: initialRoute,
      routes: routes,
    );
  }
}
