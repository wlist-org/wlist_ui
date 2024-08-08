import 'package:flutter/material.dart';

class LoadingText extends StatelessWidget {
  const LoadingText({
    super.key,
    this.value,
    this.text,
  });

  final double? value;
  final String? text;

  @override
  Widget build(BuildContext context) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        CircularProgressIndicator(
          value: value,
        ),
        if (text != null)
          Container(
            padding: const EdgeInsets.only(top: 10),
            child: Text(text!),
          )
      ],
    );
  }
}
