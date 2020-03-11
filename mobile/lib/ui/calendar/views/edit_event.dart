import 'dart:async';
import 'package:bloom/ui/calendar/blocs/edit_event.dart';
import 'package:flutter/material.dart';
import 'package:bloom/core/calendar/models.dart';
import 'package:bloom/libs/date_range_picker.dart' as date_range_picker;
import 'package:intl/intl.dart';

class EditEventView extends StatefulWidget {
  const EditEventView({this.event});

  final Event event;

  @override
  _EditEventState createState() => _EditEventState();
}

class _EditEventState extends State<EditEventView> {
  EditEventBloc _bloc;
  final TextEditingController _titleController = TextEditingController();
  final TextEditingController _descriptionController = TextEditingController();
  DateTime _startAt;
  DateTime _endAt;
  final DateFormat _dateFormatter = DateFormat('yyyy-MM-dd');

  @override
  void initState() {
    final Event event = widget.event ?? Event();

    _bloc = EditEventBloc(event: event);

    _startAt = _bloc.event.startAt;
    _endAt = _bloc.event.endAt;
    _titleController.text = _bloc.event.title;
    _descriptionController.text = _bloc.event.description;

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
                leading: IconButton(
                  icon: Icon(Icons.close),
                  onPressed: () => Navigator.of(context).pop(),
                ),
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
                decoration: const InputDecoration(labelText: 'Title'),
                controller: _titleController,
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
              onTap: () async {
                final List<DateTime> picked =
                    await date_range_picker.showDatePicker(
                  context: context,
                  initialFirstDate: _startAt,
                  initialLastDate: _endAt,
                  firstDate: DateTime(1900),
                  lastDate: DateTime(3000),
                );
                if (picked != null && picked.isNotEmpty) {
                  DateTime startAt = picked[0];
                  startAt = startAt.add(startAt.timeZoneOffset).toUtc();
                  DateTime endAt = startAt;
                  if (picked.length == 2) {
                    endAt = picked[1];
                    endAt = endAt.add(endAt.timeZoneOffset).toUtc();
                  }
                  setState(() {
                    _startAt = startAt;
                    _endAt = endAt;
                  });
                  debugPrint('dates picked: $startAt, $endAt');
                }
              },
            ),
            const SizedBox(height: 25.0),
            TextFormField(
              maxLines: 8,
              decoration: const InputDecoration(hintText: 'Description'),
              controller: _descriptionController,
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
    return <Widget>[
      FlatButton(
        onPressed: _onSavePressed,
        child: const Text('Save', style: TextStyle(color: Colors.white)),
      )
    ];
  }

  Future<void> _onSavePressed() async {
    final Event ret = await _bloc.save(
      _titleController.text,
      _descriptionController.text,
      _startAt,
      _endAt,
    );
    Navigator.of(context).pop(ret);
  }
}
