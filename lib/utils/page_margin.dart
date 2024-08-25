import 'dart:math';

import 'package:flutter/cupertino.dart';

double widthMax(BuildContext context, double percent, double width) => max(MediaQuery.sizeOf(context).width * percent, width);
double widthMin(BuildContext context, double percent, double width) => min(MediaQuery.sizeOf(context).width * percent, width);
double heightMax(BuildContext context, double percent, double height) => max(MediaQuery.sizeOf(context).height * percent, height);
double heightMin(BuildContext context, double percent, double height) => min(MediaQuery.sizeOf(context).height * percent, height);
