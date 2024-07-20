import 'package:flutter/material.dart';

class InputTextbox extends StatefulWidget {
  const InputTextbox({super.key, required this.labelText, this.hintText, this.icon, this.backgroundColor, this.password = false, required this.controller});

  final Color? backgroundColor;
  final String labelText;
  final String? hintText;
  final Icon? icon;
  final bool password;
  final TextEditingController controller;

  @override
  State<StatefulWidget> createState() => _InputTextboxState();
}

class _InputTextboxState extends State<InputTextbox> {
  bool passwordVisible = false;

  @override
  Widget build(BuildContext context) {
    return Material(
      color: widget.backgroundColor ?? (Theme.of(context).brightness == Brightness.light ? Colors.white : Colors.black),
      borderRadius: BorderRadius.circular(10),
      child: Padding(
        padding: const EdgeInsets.symmetric(vertical: 4, horizontal: 12),
        child: TextField(
          decoration: InputDecoration(
            labelText: widget.labelText,
            hintText: widget.hintText,
            icon: widget.icon,
            border: InputBorder.none,
            suffixIcon: widget.password ? IconButton(
                icon: Icon(passwordVisible ? Icons.visibility : Icons.visibility_off),
                onPressed: () => setState(() => passwordVisible = !passwordVisible)
            ) : null,
          ),
          obscureText: widget.password && !passwordVisible,
        ),
      ),
    );
  }
}
