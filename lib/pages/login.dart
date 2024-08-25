import 'package:flutter/material.dart';
import 'package:window_manager/window_manager.dart';
import 'package:wlist_ui/main.dart';
import 'package:wlist_ui/widgets/input_textbox.dart';
import 'package:wlist_ui/utils/page_margin.dart';

import '../generated/l10n.dart';

class LoginPage extends StatefulWidget {
  const LoginPage({super.key});

  void initWindow() {
    if (isDesktop) {
      windowManager.setMinimumSize(const Size(300, 400));
      windowManager.setSize(const Size(480, 600), animate: true);
    }
  }

  @override
  State<LoginPage> createState() => _LoginPageState();
}

class _LoginPageState extends State<LoginPage> {
  final InputTextStatus passportStatus = InputTextStatus();
  final InputTextStatus passwordStatus = InputTextStatus();

  bool _processing = false;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Theme.of(context).canvasColor,
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.spaceEvenly,
          children: [
            Container(
              margin: EdgeInsets.symmetric(vertical: heightMax(context, 0.04, 24)),
              child: Text(
                S.of(context).login,
                style: Theme.of(context).textTheme.headlineLarge,
              )
            ),
            Column(
              children: [
                Container(
                  margin: EdgeInsets.symmetric(horizontal: widthMax(context, 0.12, 24)) + EdgeInsets.only(bottom: heightMax(context, 0.04, 16)),
                  child: InputTextbox(
                    labelText: S.of(context).login_passport,
                    hintText: S.of(context).login_passport_hint,
                    icon: const Icon(Icons.account_circle),
                    status: passportStatus,
                    checker: (value) => value.isEmpty ? S.of(context).login_passport_empty : null,
                    enabled: !_processing,
                  ),
                ),
                Container(
                  margin: EdgeInsets.symmetric(horizontal: widthMax(context, 0.12, 24)) + EdgeInsets.only(bottom: heightMax(context, 0.04, 16)),
                  child: InputTextbox(
                    labelText: S.of(context).login_password,
                    hintText: S.of(context).login_password_hint,
                    icon: const Icon(Icons.password_rounded),
                    password: true,
                    status: passwordStatus,
                    checker: (value) => value.isEmpty ? S.of(context).login_password_empty : null,
                    enabled: !_processing,
                  ),
                ),
              ]
            ),
            OutlinedButton(
              onPressed: _processing ? null : () {
                bool passport = passportStatus.isError();
                bool password = passwordStatus.isError();
                if (!passport && !password) {
                  setState(() => _processing = true);
                  final String passport = passportStatus.text;
                  final String password = passwordStatus.text;
                  _doLogin(passport, password);
                }
              },
              child: Text(
                  S.of(context).login_button,
                  style: Theme.of(context).textTheme.bodyMedium,
              ),
            ),
            Row(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                TextButton(
                  onPressed: _doRegister,
                  child: Text(
                    S.of(context).login_register,
                    style: Theme.of(context).textTheme.bodySmall,
                  ),
                ),
                Container(
                  margin: EdgeInsets.symmetric(horizontal: widthMin(context, 0.02, 12)),
                  child: const Text("|"),
                ),
                TextButton(
                  onPressed: _doForgetPassword,
                  child: Text(
                    S.of(context).login_forget_password,
                    style: Theme.of(context).textTheme.bodySmall,
                  ),
                ),
              ],
            )
         ],
        ),
      )
    );
  }

  void _doLogin(String passport, String password) async {
    // TODO
    print(passport);
    print(password);
    Future.delayed(const Duration(seconds: 3), () => setState(() => _processing = false));
  }

  void _doRegister() async {
    // TODO
  }

  void _doForgetPassword() async {
    // TODO
  }
}
