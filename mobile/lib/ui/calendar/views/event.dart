import 'package:bloom/ui/calendar/blocs/event.dart';
import 'package:flutter/material.dart';
import 'package:bloom/core/calendar/models.dart';
import 'package:intl/intl.dart';

import 'edit_event.dart';

enum NoteViewResult {
  Deleted,
}

enum PopupMenuAction {
  Deleted,
}

class EventView extends StatefulWidget {
  const EventView({@required this.event});

  final Event event;

  @override
  _EventState createState() => _EventState();
}

class _EventState extends State<EventView> {
  EventBloc _bloc;
  final TextEditingController _titleController = TextEditingController();
  final TextEditingController _descriptionController = TextEditingController();
  DateTime _startAt;
  DateTime _endAt;
  final DateFormat _dateFormatter = DateFormat('yyyy-MM-dd');

  @override
  void initState() {
    _bloc = EventBloc(event: widget.event);

    _bloc.deletedStream.listen((_) {
      Navigator.of(context).pop();
      Navigator.of(context).pop();
    });

    super.initState();
  }

  @override
  void dispose() {
    _bloc.dispose();
    _descriptionController.dispose();
    _titleController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    _startAt = _bloc.event.startAt;
    _endAt = _bloc.event.endAt;
    _titleController.text = _bloc.event.title;
    _descriptionController.text = _bloc.event.description;

    return StreamBuilder<Event>(
        initialData: _bloc.event,
        stream: _bloc.eventStream,
        builder: (BuildContext context, AsyncSnapshot<Event> snapshot) {
          if (snapshot.hasData) {
            final Event event = snapshot.data;
            return Scaffold(
              appBar: AppBar(
                actions: _buildAppBarActions(context, event),
                elevation: 1,
              ),
              body: _buildBody(context, event),
            );
          } else {
            return Container();
          }
        });
  }

  Widget _buildBody(BuildContext context, Event note) {
    return Container(
      padding: const EdgeInsets.only(left: 16, right: 16, top: 12),
      child: SafeArea(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.start,
          children: <Widget>[
            Container(
              padding: const EdgeInsets.all(5),
              child: TextFormField(
                decoration: const InputDecoration(
                  labelText: 'Title',
                  border: InputBorder.none,
                ),
                controller: _titleController,
                readOnly: true,
              ),
            ),
            const SizedBox(height: 25.0),
            TextField(
              readOnly: true,
              style: TextStyle(
                fontSize: 25.0,
                color: Colors.blueAccent,
              ),
              decoration: InputDecoration(
                border: InputBorder.none,
                prefixIcon: const Icon(Icons.calendar_today),
                hintText:
                    '${_dateFormatter.format(_startAt)} - ${_dateFormatter.format(_endAt)}',
              ),
            ),
            const SizedBox(height: 25.0),
            TextFormField(
              maxLines: 8,
              decoration: const InputDecoration(
                labelText: 'Description',
                border: InputBorder.none,
              ),
              controller: _descriptionController,
              readOnly: true,
            ),
          ],
        ),
        left: true,
        right: true,
        top: false,
        bottom: true,
      ),
    );
  }

  List<Widget> _buildAppBarActions(BuildContext context, Event event) {
    final List<Widget> list = <Widget>[
      IconButton(
        icon: Icon(
          Icons.edit,
          color: Colors.white,
        ),
        onPressed: () => _onEditPressed(context),
      ),
      PopupMenuButton<PopupMenuAction>(
        tooltip: 'Edit event',
        itemBuilder: (BuildContext context) =>
            <PopupMenuEntry<PopupMenuAction>>[
          const PopupMenuItem<PopupMenuAction>(
            value: PopupMenuAction.Deleted,
            child: Text('Delete'),
          ),
        ],
        onSelected: (PopupMenuAction value) {
          switch (value) {
            case PopupMenuAction.Deleted:
              _deleteEvent();
          }
        },
      ),
    ];
    return list;
  }

  void _deleteEvent() {
    showDialog<Widget>(
        context: context,
        builder: (BuildContext context) {
          return AlertDialog(
            title: const Text('Confirm ?'),
            content: const Text('This event will be permanently deleted'),
            actions: <Widget>[
              FlatButton(
                onPressed: () {
                  // close dialog
                  Navigator.of(context).pop();
                },
                child: const Text('No'),
              ),
              FlatButton(
                onPressed: _bloc.delete,
                child: const Text('Yes'),
              ),
            ],
          );
        });
  }

  Future<void> _onEditPressed(BuildContext context) async {
    final Event res = await Navigator.push<dynamic>(
      context,
      MaterialPageRoute<dynamic>(
        builder: (BuildContext context) => EditEventView(event: _bloc.event),
      ),
    );

    if (res != null) {
      _bloc.eventUpdated(res);
    }
  }
}
