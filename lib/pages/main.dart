import 'package:flutter/material.dart';
import 'package:wlist_ui/utils/page_margin.dart';
import 'package:wlist_ui/utils/utils.dart';

class MainPage extends StatefulWidget {
  const MainPage({super.key});

  void initWindow() {
    setWindowSize(
      min: const Size(600, 400), // TODO: min windows size
      current: const Size(1920, 1080),
    );
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
