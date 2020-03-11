import 'dart:async';

import 'package:bloom/ui/notes/blocs/note.dart';
import 'package:bloom/core/notes/models.dart';
import 'package:bloom/ui/notes/widgets/bottom_sheet.dart';
import 'package:flutter/material.dart';
import 'package:share/share.dart';

enum NoteViewResult {
  Archived,
  Unarchived,
}

class NoteView extends StatefulWidget {
  const NoteView({this.note});

  final Note note;

  @override
  _NoteState createState() => _NoteState();
}

class _NoteState extends State<NoteView> {
  final TextEditingController _titleController = TextEditingController();
  final TextEditingController _bodyController = TextEditingController();
  final FocusNode _titleFocus = FocusNode();
  final FocusNode _bodyFocus = FocusNode();
  Timer _persistenceTimer;
  NoteBloc _bloc;

  @override
  void initState() {
    _persistenceTimer =
        Timer.periodic(const Duration(seconds: 5), (Timer timer) {
      _bloc ?? _bloc.save(_titleController.text, _bodyController.text);
    });

    final Note note = widget.note ?? Note();

    _bloc = NoteBloc(note: note);

    _titleController.text = _bloc.note.title;
    _bodyController.text = _bloc.note.body;

    _bloc.deletedStream.listen((_) {
      _persistenceTimer?.cancel();
      Navigator.of(context).pop();
      Navigator.of(context).pop();
    });

    _bloc.archivedStream.listen((_) {
      _persistenceTimer?.cancel();
      Navigator.of(context).pop(NoteViewResult.Archived);
    });

    _bloc.unarchivedStream.listen((_) {
      _persistenceTimer?.cancel();
      Navigator.of(context).pop(NoteViewResult.Unarchived);
    });

    super.initState();
  }

  @override
  void dispose() {
    _bloc.dispose();
    _bodyController.dispose();
    _titleController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return StreamBuilder<Note>(
        initialData: _bloc.note,
        stream: _bloc.noteStream,
        builder: (BuildContext context, AsyncSnapshot<Note> snapshot) {
          if (snapshot.hasData) {
            final Note note = snapshot.data;
            return WillPopScope(
              child: Scaffold(
                appBar: AppBar(
                  brightness: Brightness.light,
                  leading: const BackButton(color: Colors.black),
                  actions: _buildAppBarActions(context, note),
                  elevation: 1,
                  backgroundColor: note.color,
                ),
                body: _buildBody(context, note),
              ),
              onWillPop: _readyToPop,
            );
          } else {
            return Container();
          }
        });
  }

  Widget _buildBody(BuildContext ctx, Note note) {
    return Container(
        color: note.color,
        padding: const EdgeInsets.only(left: 16, right: 16, top: 12),
        child: SafeArea(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.start,
            children: <Widget>[
              Container(
                padding: const EdgeInsets.all(5),
                child: EditableText(
                  autofocus: true,
                  enableInteractiveSelection: true,
                  maxLines: 1,
                  controller: _titleController,
                  focusNode: _titleFocus,
                  style: const TextStyle(
                    color: Colors.black,
                    fontSize: 22,
                    fontWeight: FontWeight.bold,
                  ),
                  cursorColor: Colors.blue,
                  backgroundCursorColor: Colors.blue,
                ),
              ),
              const Divider(color: Colors.grey),
              Expanded(
                child: Container(
                  padding: const EdgeInsets.all(5),
                  child: EditableText(
                    enableInteractiveSelection: true,
                    maxLines: null,
                    controller: _bodyController,
                    focusNode: _bodyFocus,
                    style: const TextStyle(color: Colors.black, fontSize: 20),
                    backgroundCursorColor: Colors.red,
                    cursorColor: Colors.blue,
                  ),
                ),
              ),
            ],
          ),
          left: true,
          right: true,
          top: false,
          bottom: true,
        ));
  }

  List<Widget> _buildAppBarActions(BuildContext context, Note note) {
    final List<Widget> list = <Widget>[
      IconButton(
        icon: Container(
          child: Image.asset(note.isPinned
              ? 'assets/images/pin_36.png'
              : 'assets/images/pin_outline_36.png'),
          width: 20,
        ),
        onPressed: _bloc.pinUnpin,
      ),
      IconButton(
        icon: note.archivedAt == null
            ? Icon(
                Icons.archive,
                color: Colors.black,
              )
            : Icon(
                Icons.unarchive,
                color: Colors.black,
              ),
        onPressed: note.archivedAt == null ? _bloc.archive : _bloc.unarchive,
      ),
      IconButton(
        icon: Icon(
          Icons.more_vert,
          color: Colors.black,
        ),
        onPressed: () => _bottomSheet(context, note),
      ),
    ];
    final List<Widget> actions = list;
    return actions;
  }

  void _bottomSheet(BuildContext context, Note note) {
    showModalBottomSheet<MoreOptionsSheet>(
      context: context,
      builder: (BuildContext ctx) {
        return MoreOptionsSheet(
          color: note.color,
          onColorChanged: _bloc.updateColor,
          onDeleted: _onDeleted(context),
          onShared: _onShared,
          updatedAt: note.updatedAt,
        );
      },
    );
  }

  Future<bool> _readyToPop() async {
    _persistenceTimer?.cancel();
    // save data a last time before quitting
    await _bloc.save(_titleController.text, _bodyController.text);
    return true;
  }

  Function _onDeleted(BuildContext context) {
    return () {
      showDialog<Widget>(
          context: context,
          builder: (BuildContext context) {
            return AlertDialog(
              title: const Text('Confirm ?'),
              content: const Text('This note will be permanently deleted'),
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
    };
  }

  void _onShared() {
    Share.share('${_bloc.note.title}\n${_bloc.note.body}');
  }
}
