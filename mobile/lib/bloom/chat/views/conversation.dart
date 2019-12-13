// import 'package:flutter/material.dart';

// class ConversationView extends StatefulWidget {
//   const ConversationView();

//   @override
//   _ChatState createState() => _ChatState();
// }

// class _ChatState extends State<ConversationView> {
//   @override
//   Widget build(BuildContext context) {
//     return Scaffold(
//       appBar: AppBar(
//         title: const Text('Chat'),
//       ),
//       body: _buildBody(),
//     );
//   }

//   Widget _buildBody() {
//     return Widget(child: const Text('Chat'));
//   }
// }

import 'package:flutter/material.dart';
import 'package:intl/intl.dart';

class ChatEntity {
  ChatEntity({this.name, this.message, this.time, this.avatarUrl});

  final String name;
  final String message;
  final String time;
  final String avatarUrl;
}

class ChatMessage {
  ChatMessage({this.message, this.time});

  final String message;
  final String time;
}

class ConversationView extends StatefulWidget {
  const ConversationView({Key key, @required this.chatEntity})
      : super(key: key);

  final ChatEntity chatEntity;

  @override
  State<StatefulWidget> createState() => _ConversationViewState();
}

class _ConversationViewState extends State<ConversationView> {
  TextEditingController _textEditingController;
  ScrollController _scrollController;
  List<ChatMessage> _chatMessages;

  @override
  void initState() {
    super.initState();

    _textEditingController = TextEditingController();
    _scrollController = ScrollController();
    _chatMessages = <ChatMessage>[];
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: _buildAppBarTitle(),
        actions: _buildAppBarActionButtons(),
        titleSpacing: 0,
        elevation: 0.7,
        automaticallyImplyLeading: false,
      ),
      body: _buildBuildBody(),
    );
  }

  // primeiro cria uma app bar contendo somente o title e os actions, depois altera para poder adicionar a foto
  Widget _buildAppBarTitle() {
    return Row(
      mainAxisAlignment: MainAxisAlignment.start,
      crossAxisAlignment: CrossAxisAlignment.center,
      children: <Widget>[
        IconButton(
          icon: const Icon(Icons.arrow_back),
          onPressed: () {
            Navigator.pop(context);
          },
        ),
        CircleAvatar(
          foregroundColor: Theme.of(context).primaryColor,
          backgroundColor: Colors.grey,
          backgroundImage: NetworkImage(widget.chatEntity.avatarUrl),
        ),
        const SizedBox(width: 10),
        Flexible(
          child: Text(
            widget.chatEntity.name,
            style: const TextStyle(fontSize: 19),
          ),
        ),
      ],
    );
  }

  List<IconButton> _buildAppBarActionButtons() {
    return <IconButton>[
      // IconButton(
      //   icon: Icon(Icons.videocam),
      //   padding: const EdgeInsets.all(0),
      //   onPressed: () => print('Pressed view call'),
      // ),
      // IconButton(
      //   icon: Icon(Icons.call),
      //   onPressed: () => print('Pressed phone call'),
      // ),
      IconButton(
        icon: Icon(Icons.more_vert),
        onPressed: () => print('Pressed menu'),
      )
    ];
  }

  Widget _buildBuildBody() {
    return Stack(
      fit: StackFit.expand,
      children: <Widget>[
        // Image.asset(
        //   'assets/images/chat_background.png',
        //   fit: BoxFit.cover,
        // ),
        SafeArea(
          child: Container(
            child: Column(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: <Widget>[_buildChat(), _buildTextFieldAndSendButton()],
            ),
          ),
        )
      ],
    );
  }

  Widget _buildChat() {
    return _ChatMessages(
      chatMessages: _chatMessages,
      scrollController: _scrollController,
    );
  }

  Widget _buildTextFieldAndSendButton() {
    return Padding(
      padding: const EdgeInsets.fromLTRB(8, 8, 4, 8),
      child: Row(
        children: <Widget>[
          Flexible(
            child: Container(
              decoration: BoxDecoration(
                  color: Colors.white, borderRadius: BorderRadius.circular(30)),
              child: _buildTextField(),
            ),
          ),
          const SizedBox(width: 4),
          ButtonTheme(
            minWidth: 50.0,
            height: 50.0,
            buttonColor: Theme.of(context).primaryColor,
            child: RaisedButton(
              shape: const CircleBorder(),
              onPressed: _onPressedSendMessage,
              child: Icon(
                Icons.send,
                color: Colors.white,
              ),
            ),
          )
        ],
      ),
    );
  }

  void _onPressedSendMessage() {
    if (_textEditingController.text.isEmpty) {
      return;
    }

    final DateTime now = DateTime.now();
    final DateFormat formatter = DateFormat('HH:mm');
    final String time = formatter.format(now);

    final ChatMessage chatMessage =
        ChatMessage(message: _textEditingController.text, time: time);

    _textEditingController.clear();
    _scrollController.animateTo(0,
        duration: const Duration(milliseconds: 300), curve: Curves.ease);

    setState(() {
      _chatMessages.insert(0, chatMessage);
    });
  }

  Widget _buildTextField() {
    return Row(
      children: <Widget>[
        Flexible(
          child: TextField(
            expands: false,
            minLines: 1,
            maxLines: 4,
            controller: _textEditingController,
            decoration: const InputDecoration(
              border: InputBorder.none,
              hintText: '  Type message',
              hintStyle: TextStyle(color: Colors.grey),
              contentPadding: EdgeInsets.fromLTRB(20, 10, 10, 10),
              // prefixIcon: Icon(
              //   Icons.tag_faces,
              //   color: Colors.grey,
              // ),
            ),
          ),
        ),
        // Row(
        //   mainAxisSize: MainAxisSize.min,
        //   mainAxisAlignment: MainAxisAlignment.end,
        //   crossAxisAlignment: CrossAxisAlignment.center,
        //   children: <Widget>[
        //     IconButton(
        //       splashColor: Colors.transparent,
        //       highlightColor: Colors.transparent,
        //       icon: Icon(
        //         Icons.attach_file,
        //         color: Colors.grey,
        //       ),
        //       onPressed: () => print('Pressed Attach File'),
        //     ),
        //     IconButton(
        //       splashColor: Colors.transparent,
        //       highlightColor: Colors.transparent,
        //       icon: Icon(
        //         Icons.photo_camera,
        //         color: Colors.grey,
        //       ),
        //       onPressed: () => print('Pressed Camera'),
        //     ),
        //   ],
        // )
      ],
    );
  }
}

class _ChatMessages extends StatelessWidget {
  const _ChatMessages({Key key, this.chatMessages, this.scrollController})
      : super(key: key);

  final List<ChatMessage> chatMessages;
  final ScrollController scrollController;

  @override
  Widget build(BuildContext context) {
    return Expanded(
      child: ListView.builder(
          controller: scrollController,
          reverse: true,
          itemCount: chatMessages.length,
          itemBuilder: (BuildContext context, int index) {
            return _buildBubbleMessage(chatMessages[index]);
          }),
    );
  }

  Widget _buildBubbleMessage(ChatMessage chatMessage) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.end,
      children: <Widget>[
        Container(
          margin: const EdgeInsets.fromLTRB(40, 4, 20, 10),
          padding: const EdgeInsets.all(8.0),
          decoration: BoxDecoration(
            color: const Color(0xFFE2FEC9),
            borderRadius: const BorderRadius.only(
                topRight: Radius.circular(0),
                topLeft: Radius.circular(8.0),
                bottomRight: Radius.circular(8.0),
                bottomLeft: Radius.circular(8.0)),
            boxShadow: <BoxShadow>[
              BoxShadow(
                blurRadius: .5,
                spreadRadius: 1.0,
                color: Colors.black.withOpacity(.12),
                offset: const Offset(1, 1),
              )
            ],
          ),
          child: _buildBubbleContent(chatMessage),
        ),
      ],
    );
  }

  Widget _buildBubbleContent(ChatMessage chatMessage) {
    return Stack(
      children: <Widget>[
        Padding(
          padding: const EdgeInsets.only(right: 52.0),
          child: Text(chatMessage.message,
              style: const TextStyle(
                fontSize: 18.0,
              )),
        ),
        Positioned(
          bottom: 0.0,
          right: 0.0,
          child: Row(
            children: <Widget>[
              Text(chatMessage.time,
                  style: TextStyle(
                    color: Colors.black38,
                    fontSize: 12.0,
                  )),
              const SizedBox(width: 3.0),
              Icon(
                Icons.done_all,
                size: 14.0,
                color: Colors.black38,
              )
            ],
          ),
        )
      ],
    );
  }
}
