import 'dart:math';
import 'dart:ui';

import 'package:flutter/material.dart';

double widthPercent(BuildContext context, double percent) => MediaQuery.sizeOf(context).width * percent;
double widthMax(BuildContext context, double percent, double width) => max(widthPercent(context, percent), width);
double widthMin(BuildContext context, double percent, double width) => min(widthPercent(context, percent), width);
double widthClamp(BuildContext context, double min, double max, double width) => clampDouble(width, widthPercent(context, min), widthPercent(context, max));

double heightPercent(BuildContext context, double percent) => MediaQuery.sizeOf(context).height * percent;
double heightMax(BuildContext context, double percent, double height) => max(heightPercent(context, percent), height);
double heightMin(BuildContext context, double percent, double height) => min(heightPercent(context, percent), height);
double heightClamp(BuildContext context, double min, double max, double height) => clampDouble(height, heightPercent(context, min), heightPercent(context, max));
