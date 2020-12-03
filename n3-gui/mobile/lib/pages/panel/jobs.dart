import 'package:flutter/material.dart';
import 'package:flutter_slidable/flutter_slidable.dart';
import 'package:intl/intl.dart';
import 'package:n3_mobile/models/work.dart';
import 'package:n3_mobile/pages/panel/base.dart';

class PanelJobs extends StatefulWidget implements PanelItem {
  String get label => 'jobs';
  IconData get icon => Icons.rotate_right_outlined;

  @override
  State createState() => _State();
}

class _State extends State {
  List<Work> works = Work.sample();

  @override
  Widget build(BuildContext context) {
    return ListView.builder(
      itemCount: this.works.length,
      itemBuilder: (context, index) {
        final work = works[index];

        final command = work.command.commandToString().toUpperCase();
        final exec = work.exec;

        final isError = work.status.errorMsg != null;
        final isRunning = !isError && work.status.isRunning;

        var dateEnd = work.status.dateEnd != null
            ? DateFormat("yyyy-MM-dd HH:mm:ss").format((work.status.dateEnd))
            : 'Unknown';

        final icon = isError
            ? CircleAvatar(
                backgroundColor: Colors.red,
                foregroundColor: Colors.white,
                child: Icon(Icons.priority_high),
              )
            : isRunning
                ? CircleAvatar(
                    backgroundColor: Colors.green,
                    foregroundColor: Colors.white,
                    child: Icon(Icons.check),
                  )
                : CircleAvatar(
                    backgroundColor: Colors.orange,
                    foregroundColor: Colors.white,
                    child: Icon(Icons.model_training),
                  );

        return Slidable(
          key: ValueKey(work.id),
          actionPane: SlidableDrawerActionPane(),
          secondaryActions: [
            IconSlideAction(
              caption: 'More',
              color: Colors.grey.shade200,
              icon: Icons.more_horiz,
              onTap: () => _onWorkMore(work),
            ),
            IconSlideAction(
              caption: 'Delete',
              color: Colors.red,
              icon: Icons.delete,
              onTap: () => _onWorkDelete(work),
            ),
          ],
          // dismissal: SlidableDismissal(
          //   child: SlidableDrawerDismissal(),
          //   onDismissed: (_) => _onWorkDelete(work),
          // ),
          child: ListTile(
            leading: icon,
            title: Text('[$command] $exec'),
            subtitle: Text('ETA $dateEnd'),
            onTap: () => _onWorkMore(work),
          ),
        );
      },
    );
  }

  void _onWorkMore(final Work work) {
    print(work.toJson());
  }

  void _onWorkDelete(final Work work) {
    print(work.toJson());
  }
}
