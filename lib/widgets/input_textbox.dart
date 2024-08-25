import 'package:flutter/material.dart';

class InputTextStatus {
  _InputTextboxState? _value;
  _InputTextboxState get _status {
    assert(_value != null, "Please make sure to wire it up to the textbox.");
    return _value!;
  }

  String get text => _status.controller.text;
  set text(String text) => _status.controller.text = text;
  bool isError() => _status.isError();
}

class InputTextbox extends StatefulWidget {
  const InputTextbox({
    super.key,
    required this.labelText,
    this.hintText,
    this.icon,
    this.backgroundColor,
    this.password = false,
    this.checker,
    this.status,
    this.enabled = true,
  });

  final Color? backgroundColor;
  final String labelText;
  final String? hintText;
  final Icon? icon;
  final bool password;
  final String? Function(String)? checker;
  final InputTextStatus? status;
  final bool enabled;

  @override
  State<StatefulWidget> createState() => _InputTextboxState();
}

class _InputTextboxState extends State<InputTextbox> {
  final TextEditingController controller = TextEditingController();

  bool passwordVisible = false;
  String? errorText;

  @override
  void initState() {
    super.initState();
    if (widget.status != null) {
      widget.status!._value = this;
    }
  }

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
            errorText: errorText,
            icon: widget.icon,
            border: InputBorder.none,
            suffixIcon: widget.password ? IconButton(
                icon: Icon(passwordVisible ? Icons.visibility : Icons.visibility_off),
                onPressed: () => setState(() => passwordVisible = !passwordVisible)
            ) : null,
          ),
          controller: controller,
          obscureText: widget.password && !passwordVisible,
          onChanged: widget.checker == null ? null : (value) {
            String? error = widget.checker!(value);
            if (errorText != error) {
              setState(() => errorText = error);
            }
          },
          enabled: widget.enabled,
        ),
      ),
    );
  }

  @override
  void dispose() {
    controller.dispose();
    super.dispose();
  }

  bool isError() {
    if (errorText != null) return true;
    if (widget.checker == null) return false;
    String? error = widget.checker!(controller.text);
    if (errorText != error) {
      setState(() => errorText = error);
    }
    return error != null;
  }
}
