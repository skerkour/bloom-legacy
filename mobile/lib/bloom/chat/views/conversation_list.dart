import 'package:bloom/bloom/chat/views/conversation.dart';
import 'package:flutter/material.dart';

class ConversationListView extends StatefulWidget {
  const ConversationListView({Key key}) : super(key: key);

  @override
  _ConversationListViewState createState() => _ConversationListViewState();
}

class _ConversationListViewState extends State<ConversationListView> {
  static final List<_Conversation> _conversations = getConversations();

  @override
  Widget build(BuildContext context) {
    return ListView.separated(
      itemCount: _conversations.length,
      separatorBuilder: (BuildContext context, int index) => const Divider(),
      itemBuilder: (BuildContext context, int i) {
        return ListTile(
          leading: CircleAvatar(
            foregroundColor: Theme.of(context).primaryColor,
            backgroundColor: Colors.grey,
            backgroundImage: AssetImage(_conversations[i].avatar),
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
                style: const TextStyle(color: Colors.grey, fontSize: 14.0),
              ),
            ],
          ),
          subtitle: Container(
            padding: const EdgeInsets.only(top: 5.0),
            child: Text(
              _conversations[i].message,
              style: const TextStyle(color: Colors.grey, fontSize: 15.0),
            ),
          ),
          onTap: () {
            final ChatEntity entity = ChatEntity(
                name: _conversations[i].name,
                message: _conversations[i].message,
                time: '15:30',
                avatarUrl: _conversations[i].avatar);
            Navigator.push<dynamic>(
              context,
              MaterialPageRoute<dynamic>(
                builder: (BuildContext context) =>
                    ConversationView(chatEntity: entity),
              ),
            );
          },
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

const String _avatar = 'assets/images/sylvain.jpg';

List<_Conversation> getConversations() {
  return <_Conversation>[
    _Conversation(
        avatar: _avatar, name: 'Sylvain Kerkour', message: 'Hello world!'),
    _Conversation(
        avatar: _avatar, name: 'Sylvain Kerkour', message: 'Hello world!'),
    _Conversation(
        avatar: _avatar, name: 'Sylvain Kerkour', message: 'Hello world!'),
    _Conversation(
        avatar: _avatar, name: 'Sylvain Kerkour', message: 'Hello world!'),
    _Conversation(
        avatar: _avatar, name: 'Sylvain Kerkour', message: 'Hello world!'),
    _Conversation(
        avatar: _avatar, name: 'Sylvain Kerkour', message: 'Hello world!'),
    _Conversation(
        avatar: _avatar, name: 'Sylvain Kerkour', message: 'Hello world!'),
    _Conversation(
        avatar: _avatar, name: 'Sylvain Kerkour', message: 'Hello world!'),
    _Conversation(
        avatar: _avatar, name: 'Sylvain Kerkour', message: 'Hello world!'),
    _Conversation(
        avatar: _avatar, name: 'Sylvain Kerkour', message: 'Hello world!'),
    _Conversation(
        avatar: _avatar, name: 'Sylvain Kerkour', message: 'Hello world!'),
    _Conversation(
        avatar: _avatar, name: 'Sylvain Kerkour', message: 'Hello world!'),
    _Conversation(
        avatar: _avatar, name: 'Sylvain Kerkour', message: 'Hello world!'),
    _Conversation(
        avatar: _avatar, name: 'Sylvain Kerkour', message: 'Hello world!'),
  ];
}

String timeToString(DateTime time) {
  return '${time.hour}:${time.minute}';
}
