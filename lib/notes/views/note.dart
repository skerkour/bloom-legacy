import 'dart:async';

import 'package:bloom/notes/blocs/note_bloc.dart';
import 'package:bloom/notes/models/db/note.dart';
import 'package:bloom/notes/widgets/bottom_sheet.dart';
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
  // String _initialTitle;
  // String _initialBody;
  Timer _persistenceTimer;
  NoteBloc _bloc;
  Color _color;

  @override
  void initState() {
    _persistenceTimer =
        Timer.periodic(const Duration(seconds: 5), (Timer timer) {
      if (_bloc != null) {
        _bloc.save(_titleController.text, _bodyController.text);
      }
    });

    final Note note = widget.note ?? Note();

    _bloc = NoteBloc(note: note);
    _color = note.color;

    _titleController.text = _bloc.note.title;
    _bodyController.text = _bloc.note.body;

    _bloc.deleted.listen((Note _) {
      Navigator.of(context).pop();
      Navigator.of(context).pop();
      _persistenceTimer?.cancel();
    });

    _bloc.archived.listen((Note _) {
      _persistenceTimer?.cancel();
      Navigator.of(context).pop(NoteViewResult.Archived);
    });

    _bloc.unarchived.listen((Note _) {
      _persistenceTimer?.cancel();
      Navigator.of(context).pop(NoteViewResult.Unarchived);
    });

    _bloc.color.listen((Color color) {
      setState(() {
        _color = color;
      });
    });

    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return WillPopScope(
      child: Scaffold(
        appBar: AppBar(
          brightness: Brightness.light,
          leading: BackButton(
            color: Colors.black,
          ),
          actions: _buildAppBarActions(context),
          elevation: 1,
          backgroundColor: _color,
        ),
        body: Builder(builder: (BuildContext context) {
          return _buildBody(context);
        }),
      ),
      onWillPop: _readyToPop,
    );
  }

  Widget _buildBody(BuildContext ctx) {
    return Container(
        color: _color,
        padding: const EdgeInsets.only(left: 16, right: 16, top: 12),
        child: SafeArea(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.start,
            children: <Widget>[
              Container(
                padding: const EdgeInsets.all(5),
                child: EditableText(
                  maxLines: 1,
                  controller: _titleController,
                  focusNode: _titleFocus,
                  style: TextStyle(
                    color: Colors.black,
                    fontSize: 22,
                    fontWeight: FontWeight.bold,
                  ),
                  cursorColor: Colors.blue,
                  backgroundCursorColor: Colors.blue,
                ),
              ),
              Divider(color: Colors.grey),
              Expanded(
                child: Container(
                  padding: const EdgeInsets.all(5),
                  child: EditableText(
                    maxLines: null,
                    controller: _bodyController,
                    focusNode: _bodyFocus,
                    style: TextStyle(color: Colors.black, fontSize: 20),
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

  List<Widget> _buildAppBarActions(BuildContext context) {
    final List<Widget> list = <Widget>[
      Padding(
        padding: const EdgeInsets.symmetric(horizontal: 12),
        child: InkWell(
          child: GestureDetector(
            child:
                Container(child: Image.asset('assets/pin_36.png'), width: 20),
          ),
        ),
      ),
      Padding(
        padding: const EdgeInsets.symmetric(horizontal: 12),
        child: _bloc.note.archivedAt == null
            ? GestureDetector(
                onTap: _bloc.archive,
                child: Icon(
                  Icons.archive,
                  color: Colors.black,
                ),
              )
            : GestureDetector(
                onTap: _bloc.unarchive,
                child: Icon(
                  Icons.unarchive,
                  color: Colors.black,
                ),
              ),
      ),
      Padding(
        padding: const EdgeInsets.symmetric(horizontal: 12),
        child: InkWell(
          child: GestureDetector(
            onTap: () => _bottomSheet(context),
            child: Icon(
              Icons.more_vert,
              color: Colors.black,
            ),
          ),
        ),
      ),
    ];
    final List<Widget> actions = list;
    return actions;
  }

  void _bottomSheet(BuildContext context) {
    showModalBottomSheet<MoreOptionsSheet>(
      context: context,
      builder: (BuildContext ctx) {
        return MoreOptionsSheet(
          color: _color,
          onColorChanged: _bloc.updateColor,
          onDeleted: _onDeleted(context),
          onShared: _onShared,
          updatedAt: _bloc.note.updatedAt,
        );
      },
    );
  }

  Future<bool> _readyToPop() async {
    _persistenceTimer?.cancel();
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

  @override
  void dispose() {
    _bloc.dispose();
    super.dispose();
  }
}
