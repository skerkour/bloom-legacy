import 'package:bloom/ui/notes/blocs/notes.dart';
import 'package:bloom/core/notes/models.dart';
import 'package:bloom/ui/notes/views/note.dart';
import 'package:bloom/ui/notes/widgets/drawer.dart';
import 'package:bloom/ui/notes/widgets/staggered_tile.dart';
import 'package:flutter/material.dart';
import 'package:flutter_staggered_grid_view/flutter_staggered_grid_view.dart';

class NotesView extends StatefulWidget {
  const NotesView({this.archive = false});
  final bool archive;

  @override
  _NotesState createState() => _NotesState();
}

class _NotesState extends State<NotesView> {
  NotesBloc _bloc;
  bool _archive;

  @override
  void initState() {
    _bloc = NotesBloc();
    _archive = widget.archive ?? false;
    super.initState();
  }

  @override
  void dispose() {
    _bloc.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    if (_archive) {
      _bloc.getArchive();
    } else {
      _bloc.getNotes();
    }
    return Scaffold(
      endDrawer: const NotesDrawer(),
      appBar: AppBar(
        title: Text(_archive ? 'Archive' : 'Notes'),
      ),
      body: StreamBuilder<List<Note>>(
        stream: _bloc.notesOut,
        builder: (BuildContext context, AsyncSnapshot<List<Note>> snapshot) {
          if (snapshot.hasData) {
            return _buildBody(context, snapshot.data);
          } else {
            return const Center(child: CircularProgressIndicator());
          }
        },
      ),
      floatingActionButton: _archive
          ? null
          : FloatingActionButton(
              onPressed: () => _newNoteTapped(context),
              child: Icon(Icons.add),
              backgroundColor: Colors.red,
            ),
    );
  }

  Widget _buildBody(BuildContext ctx, List<Note> notes) {
    return Container(
      child: Padding(
        padding: _paddingForView(context),
        child: StaggeredGridView.count(
          crossAxisSpacing: 6,
          mainAxisSpacing: 6,
          crossAxisCount: _colForStaggeredView(context),
          children: List<BlmStaggeredTile>.generate(notes.length, (int i) {
            return BlmStaggeredTile(note: notes[i]);
          }),
          staggeredTiles: _tilesForView(notes),
        ),
      ),
    );
  }

  int _colForStaggeredView(BuildContext context) {
    // for width larger than 600 on grid mode, return 3 irrelevant of the orientation to accommodate more notes horizontally
    return MediaQuery.of(context).size.width > 600 ? 3 : 2;
  }

  List<StaggeredTile> _tilesForView(List<Note> notes) {
    // Generate staggered tiles for the view based on the current preference.
    return List<StaggeredTile>.generate(notes.length, (int index) {
      return const StaggeredTile.fit(1);
    });
  }

  EdgeInsets _paddingForView(BuildContext context) {
    final double width = MediaQuery.of(context).size.width;
    double leftAndRight;
    const double bottomAndTop = 8;
    if (width > 500) {
      leftAndRight = width * 0.05; // 5% padding of width on both side
    } else {
      leftAndRight = 8;
    }
    return EdgeInsets.only(
        left: leftAndRight,
        right: leftAndRight,
        top: bottomAndTop,
        bottom: bottomAndTop);
  }

  Future<void> _newNoteTapped(BuildContext ctx) async {
    debugPrint('new note tapped');
    // Navigator.pushNamed(ctx, '/notes/note', arguments: Note());
    final NoteViewResult res = await Navigator.push<dynamic>(
      context,
      MaterialPageRoute<dynamic>(
        builder: (BuildContext context) => const NoteView(),
      ),
    );

    if (res != null) {
      Scaffold.of(context)
        ..removeCurrentSnackBar()
        ..showSnackBar(
          SnackBar(content: Text('Note ${res.toString().split('.').last}')),
        );
    }
  }
}
