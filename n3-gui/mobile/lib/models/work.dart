import 'package:flutter/material.dart';

class Work {
  final Command command;
  final String exec;
  final Map<String, String> variables;

  final WorkStatus status;

  BigInt get id => this.status.id;

  const Work({
    @required this.command,
    @required this.exec,
    @required this.variables,
    @required this.status,
  });

  static Work fromJson(Map<String, dynamic> source) {
    return Work(
      command: CommandToString.commandFromString(source['command']),
      exec: source['exec'],
      variables: source['variables'],
      status: WorkStatus.fromJson(source['status']),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'command': this.command.commandToString(),
      'exec': this.exec,
      'variables': this.variables,
      'status': this.status.toJson(),
    };
  }

  static List<Work> sample() {
    return [
      Work(
        command: CommandToString.commandFromString('train'),
        exec: 'ImageClassification',
        variables: {},
        status: WorkStatus(
          id: BigInt.from(123),
          isRunning: false,
          dateBegin: DateTime.now(),
          dateEnd: DateTime.now(),
        ),
      ),
      Work(
        command: CommandToString.commandFromString('train'),
        exec: 'ImageClassification',
        variables: {},
        status: WorkStatus(
          id: BigInt.from(124),
          isRunning: true,
          dateBegin: DateTime.now(),
        ),
      ),
      Work(
        command: CommandToString.commandFromString('train'),
        exec: 'ImageClassification',
        variables: {},
        status: WorkStatus(
          id: BigInt.from(125),
          isRunning: false,
          errorMsg: 'ERROR!',
          dateBegin: DateTime.now(),
        ),
      ),
    ];
  }
}

class WorkStatus {
  final BigInt id;
  final bool isRunning;
  final String errorMsg;

  // Local Time (not UTC)
  final DateTime dateBegin;
  final DateTime dateEnd;

  const WorkStatus({
    @required this.id,
    @required this.isRunning,
    this.errorMsg,
    @required this.dateBegin,
    this.dateEnd,
  });

  static WorkStatus fromJson(Map<String, dynamic> source) {
    return WorkStatus(
      id: BigInt.from(source['id']),
      isRunning: source['is_running'],
      errorMsg: source['error_msg'],
      // UTC -> Local
      dateBegin: DateTime.tryParse(source['date_begin'])?.toLocal(),
      dateEnd: DateTime.tryParse(source['date_end'])?.toLocal(),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'command': this.id.toString(),
      'is_running': this.isRunning,
      'error_msg': this.errorMsg,
      // Local -> UTC
      'date_begin': this.dateBegin?.toUtc().toString(),
      'date_end': this.dateEnd?.toUtc().toString(),
    };
  }
}

enum Command {
  Unknown,
  Train,
  Eval,
}

extension CommandToString on Command {
  static Command commandFromString(String command) {
    switch (command) {
      case 'train':
        return Command.Train;
      case 'eval':
        return Command.Eval;
      case '':
      default:
        return Command.Unknown;
    }
  }

  String commandToString() {
    switch (this) {
      case Command.Train:
        return 'train';
      case Command.Eval:
        return 'eval';
      case Command.Unknown:
      default:
        return '';
    }
  }
}
