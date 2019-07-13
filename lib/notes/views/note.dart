import 'dart:async';

import 'package:bloom/notes/models/db/note.dart';
import 'package:flutter/material.dart';

class NoteView extends StatefulWidget {
  const NoteView(this.note);

  final Note note;

  @override
  _NotesState createState() => _NotesState();
}

class _NotesState extends State<NoteView> {
  final TextEditingController _titleController = TextEditingController();
  final TextEditingController _bodyController = TextEditingController();
  final FocusNode _titleFocus = FocusNode();
  final FocusNode _bodyFocus = FocusNode();
  // String _initialTitle;
  // String _initialBody;
  Timer _persistenceTimer;
  Color _color;
  Note _note;

  @override
  void initState() {
    _note = widget.note;
    _titleController.text = _note.title;
    _bodyController.text = _note.body;

    // _initialTitle = _note.title;
    // _initialBody = _note.body;
    _color = _note.color;

    _persistenceTimer = Timer.periodic(Duration(seconds: 5), (Timer timer) {
      // call insert query here
      print('5 seconds passed');
      print('editable note id: ${_note.id}');
      _persistData();
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
          backgroundColor: Colors.white,
        ),
        body: _buildBody(context),
      ),
      onWillPop: _readyToPop,
    );
  }

  Widget _buildBody(BuildContext ctx) {
    return Container(
        color: Colors.white,
        padding: const EdgeInsets.only(left: 16, right: 16, top: 12),
        child: SafeArea(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.start,
            children: <Widget>[
              Flexible(
                child: Container(
                  padding: const EdgeInsets.all(5),
//          decoration: BoxDecoration(border: Border.all(color: CentralStation.borderColor,width: 1 ),borderRadius: BorderRadius.all(Radius.circular(10)) ),
                  child: EditableText(
                      // onChanged: (str) => {updateNoteObject()},
                      maxLines: null,
                      controller: _titleController,
                      focusNode: _titleFocus,
                      style: TextStyle(
                          color: Colors.black,
                          fontSize: 22,
                          fontWeight: FontWeight.bold),
                      cursorColor: Colors.blue,
                      backgroundCursorColor: Colors.blue),
                ),
              ),
              Divider(color: Colors.grey),
              Flexible(
                  child: Container(
                      padding: const EdgeInsets.all(5),
//    decoration: BoxDecoration(border: Border.all(color: CentralStation.borderColor,width: 1),borderRadius: BorderRadius.all(Radius.circular(10)) ),
                      child: EditableText(
                        maxLines: 300, // line limit extendable later
                        controller: _bodyController,
                        focusNode: _bodyFocus,
                        style: TextStyle(color: Colors.black, fontSize: 20),
                        backgroundCursorColor: Colors.red,
                        cursorColor: Colors.blue,
                      )))
            ],
          ),
          left: true,
          right: true,
          top: false,
          bottom: false,
        ));
  }

  List<Widget> _buildAppBarActions(BuildContext context) {
    final List<Widget> actions = <Widget>[
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
        child: InkWell(
          child: GestureDetector(
            child: Icon(
              Icons.archive,
              color: Colors.black,
            ),
          ),
        ),
      ),
      Padding(
        padding: const EdgeInsets.symmetric(horizontal: 12),
        child: InkWell(
          child: GestureDetector(
            // onTap: () => bottomSheet(context),
            child: Icon(
              Icons.more_vert,
              color: Colors.black,
            ),
          ),
        ),
      )
    ];
    return actions;
  }

  Future<void> _persistData() async {
    _note.body = _bodyController.text;
    _note.title = _titleController.text;
    _note.color = _color;

    if (_note.id == '') {
      _note = await Note.create(_note.title, _note.body, _note.color);
      print('note created');
    } else {
      _note = await _note.update();
      print('note saved');
    }
  }

  Future<bool> _readyToPop() async {
    _persistenceTimer.cancel();
    //show saved toast after calling _persistData function.

    await _persistData();
    return true;
  }
}
