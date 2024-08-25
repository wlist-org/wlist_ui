import 'package:flutter/material.dart';
import 'package:window_manager/window_manager.dart';
import 'package:wlist_ui/main.dart';

class MainPage extends StatefulWidget {
  const MainPage({super.key});

  void initWindow() {
    if (isDesktop) {
      windowManager.setMinimumSize(const Size(300, 400)); // TODO: windows size
      windowManager.setSize(const Size(800, 600), animate: true);
    }
  }

  @override
  State<MainPage> createState() => _MainPageState();
}

class _MainPageState extends State<MainPage> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
        body: const Center() // TODO
    );
  }
}
