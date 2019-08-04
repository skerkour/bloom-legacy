import 'package:flutter/material.dart';

class TabChatsView extends StatefulWidget {
  const TabChatsView({Key key}) : super(key: key);

  @override
  _TabChatsViewState createState() => _TabChatsViewState();
}

class _TabChatsViewState extends State<TabChatsView> {
  static final List<_Conversation> _conversations = getConversations();

  @override
  Widget build(BuildContext context) {
    return ListView.separated(
      itemCount: _conversations.length,
      separatorBuilder: (BuildContext context, int index) => Divider(),
      itemBuilder: (BuildContext context, int i) {
        return ListTile(
          leading: CircleAvatar(
            foregroundColor: Theme.of(context).primaryColor,
            backgroundColor: Colors.grey,
            backgroundImage: NetworkImage(_conversations[i].avatar),
          ),
          title: Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: <Widget>[
              Text(
                _conversations[i].name,
                // style: TextStyle(fontWeight: FontWeight.bold),
              ),
              Text(
                timeToString(_conversations[i].time),
                style: TextStyle(color: Colors.grey, fontSize: 14.0),
              ),
            ],
          ),
          subtitle: Container(
            padding: const EdgeInsets.only(top: 5.0),
            child: Text(
              _conversations[i].message,
              style: TextStyle(color: Colors.grey, fontSize: 15.0),
            ),
          ),
        );
      },
    );
  }
}

class _Conversation {
  _Conversation(
      {@required this.avatar, @required this.name, @required this.message}) {
    time = DateTime.now();
  }
  final String avatar;
  final String name;
  final String message;
  DateTime time;
}

List<_Conversation> getConversations() {
  return <_Conversation>[
    _Conversation(
        avatar: 'https://www.kerkour.fr/about/sylvain.jpg',
        name: 'Sylvain Kerkour',
        message: 'Hello world!'),
    _Conversation(
        avatar: 'https://www.kerkour.fr/about/sylvain.jpg',
        name: 'Sylvain Kerkour',
        message: 'Hello world!'),
    _Conversation(
        avatar: 'https://www.kerkour.fr/about/sylvain.jpg',
        name: 'Sylvain Kerkour',
        message: 'Hello world!'),
    _Conversation(
        avatar: 'https://www.kerkour.fr/about/sylvain.jpg',
        name: 'Sylvain Kerkour',
        message: 'Hello world!'),
    _Conversation(
        avatar: 'https://www.kerkour.fr/about/sylvain.jpg',
        name: 'Sylvain Kerkour',
        message: 'Hello world!'),
    _Conversation(
        avatar: 'https://www.kerkour.fr/about/sylvain.jpg',
        name: 'Sylvain Kerkour',
        message: 'Hello world!'),
    _Conversation(
        avatar: 'https://www.kerkour.fr/about/sylvain.jpg',
        name: 'Sylvain Kerkour',
        message: 'Hello world!'),
    _Conversation(
        avatar: 'https://www.kerkour.fr/about/sylvain.jpg',
        name: 'Sylvain Kerkour',
        message: 'Hello world!'),
    _Conversation(
        avatar: 'https://www.kerkour.fr/about/sylvain.jpg',
        name: 'Sylvain Kerkour',
        message: 'Hello world!'),
    _Conversation(
        avatar: 'https://www.kerkour.fr/about/sylvain.jpg',
        name: 'Sylvain Kerkour',
        message: 'Hello world!'),
    _Conversation(
        avatar: 'https://www.kerkour.fr/about/sylvain.jpg',
        name: 'Sylvain Kerkour',
        message: 'Hello world!'),
    _Conversation(
        avatar: 'https://www.kerkour.fr/about/sylvain.jpg',
        name: 'Sylvain Kerkour',
        message: 'Hello world!'),
    _Conversation(
        avatar: 'https://www.kerkour.fr/about/sylvain.jpg',
        name: 'Sylvain Kerkour',
        message: 'Hello world!'),
    _Conversation(
        avatar: 'https://www.kerkour.fr/about/sylvain.jpg',
        name: 'Sylvain Kerkour',
        message: 'Hello world!'),
  ];
}

String timeToString(DateTime time) {
  return '${time.hour}:${time.minute}';
}
