import 'package:flutter/material.dart';

class IntroPage extends StatefulWidget {
  @override
  State createState() => _State();
}

class _State extends State {
  @override
  void initState() {
    super.initState();
    _waitAndMoveToMainPage();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Center(
        child: Image.asset('assets/images/logo.png'),
      ),
    );
  }

  void _waitAndMoveToMainPage() {
    Future.delayed(
      const Duration(seconds: 1),
      () {
        if (!mounted) return;
        _moveToMainPage();
      },
    );
  }

  void _moveToMainPage() {
    Navigator.of(context).pushReplacementNamed('/panel');
  }
}
