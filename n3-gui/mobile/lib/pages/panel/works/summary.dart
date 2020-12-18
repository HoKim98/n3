import 'package:flutter/material.dart';
import 'package:n3_mobile/models/work.dart';
import 'package:n3_mobile/pages/panel/base.dart';
import 'package:n3_mobile/pages/panel/works/detail.dart';
import 'package:percent_indicator/percent_indicator.dart';

class WorkDetailSummary extends StatefulWidget implements PanelItem {
  String get label => 'summary';
  IconData get icon => Icons.wifi;

  final ValueNotifier<Work> work;

  WorkDetailSummary(this.work);

  @override
  State createState() => _State(work);
}

class _State extends WorkDetailState {
  _State(ValueNotifier<Work> work) : super(work);

  @override
  Widget build(BuildContext context) {
    final work = this.work.value;
    final percent = work.status.percent;

    const Color colorBegin = Colors.yellow;
    const Color colorEnd = Colors.lightGreen;
    const Color colorComplete = Colors.blueGrey;
    const Color colorError = Colors.red;

    var color, body;
    if (work.status.isRunning == false) {
      var icon, iconColor;
      if (work.status.errorMsg == null) {
        color = colorComplete;
        icon = Icons.check;
        iconColor = colorEnd;
      } else {
        color = colorError;
        icon = Icons.priority_high;
        iconColor = colorError;
      }
      body = Icon(
        icon,
        size: 70,
        color: iconColor,
      );
    } else {
      color = Color.alphaBlend(
        colorEnd.withAlpha((percent * 255).toInt()),
        colorBegin,
      );
      body = Text(
        '${(percent * 100).toInt()}%',
        style: Theme.of(context).textTheme.headline4,
      );
    }

    return Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: <Widget>[
          CircularPercentIndicator(
            progressColor: color,
            circularStrokeCap: CircularStrokeCap.round,
            radius: 200,
            lineWidth: 12,
            percent: percent,
            center: body,
          ),
        ],
      ),
    );
  }
}
