import 'package:bloom/ui/groups/views/create.dart';
import 'package:bloom/ui/home/views/chats_search.dart';
import 'package:bloom/ui/home/views/tab_groups.dart';
import 'package:bloom/ui/home/views/tab_me.dart';
import 'package:flutter/material.dart';

class HomeView extends StatefulWidget {
  const HomeView({Key key}) : super(key: key);

  @override
  _HomeViewState createState() => _HomeViewState();
}

class _HomeViewState extends State<HomeView> {
  int _selectedIndex = 0;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: _buildAppBar(context),
      body: _buildBody(context),
      bottomNavigationBar: _buildBottomNavigationBar(),
      floatingActionButton: _buildFloatingActionButton(context),
    );
  }

  AppBar _buildAppBar(BuildContext context) {
    final ThemeData theme = Theme.of(context);
    final List<IconButton> actions = <IconButton>[];

    switch (_selectedIndex) {
      // case 0:
      //   actions.add(IconButton(
      //       icon: Icon(Icons.search),
      //       onPressed: () {
      //         showSearch<String>(
      //           context: context,
      //           delegate: ChatsSearchDelegate(),
      //         );
      //       }));
      //   break;
      case 1:
        actions.add(IconButton(
            icon: Icon(Icons.search),
            onPressed: () {
              showSearch<String>(
                context: context,
                delegate: ChatsSearchDelegate(),
              );
            }));
        break;
    }

    return AppBar(
      elevation: 0,
      backgroundColor: theme.scaffoldBackgroundColor,
      iconTheme: IconThemeData(color: Colors.grey[700]),
      title: Text('Bloom', style: TextStyle(color: Colors.grey[700])),
      actions: actions,
    );
  }

  FloatingActionButton _buildFloatingActionButton(BuildContext context) {
    switch (_selectedIndex) {
      // case 0:
      //   return FloatingActionButton(
      //     onPressed: () => _fabChatsPressed(),
      //     child: Icon(Icons.add),
      //     backgroundColor: Colors.red,
      //   );
      case 1:
        return FloatingActionButton(
          onPressed: () => _fabGroupsPressed(),
          child: Icon(Icons.add),
          backgroundColor: Colors.red,
        );
      default:
        return null;
    }
  }

  // void _fabChatsPressed() {
  //   showModalBottomSheet<ChatsBottomSheet>(
  //     context: context,
  //     builder: (BuildContext ctx) {
  //       return ChatsBottomSheet();
  //     },
  //   );
  // }

  void _fabGroupsPressed() {
    Navigator.push<dynamic>(
      context,
      MaterialPageRoute<dynamic>(
        builder: (BuildContext context) => const CreateGroupView(),
      ),
    );
  }

  BottomNavigationBar _buildBottomNavigationBar() {
    return BottomNavigationBar(
      items: const <BottomNavigationBarItem>[
        // BottomNavigationBarItem(
        //   icon: Icon(Icons.chat),
        //   title: Text('Chat'),
        // ),
        BottomNavigationBarItem(
          icon: Icon(Icons.person),
          title: Text('Me'),
        ),
        BottomNavigationBarItem(
          icon: Icon(Icons.people),
          title: Text('Groups'),
        ),
      ],
      currentIndex: _selectedIndex,
      selectedItemColor: Colors.blue,
      onTap: _onItemTapped,
      type: BottomNavigationBarType.fixed,
    );
  }

  Widget _buildBody(BuildContext context) {
    switch (_selectedIndex) {
      // case 0:
      //   return const ConversationListView();
      case 1:
        return const TabGroupsView();
      default:
        return const TabMeView();
    }
  }

  void _onItemTapped(int index) {
    setState(() {
      _selectedIndex = index;
    });
  }
}
