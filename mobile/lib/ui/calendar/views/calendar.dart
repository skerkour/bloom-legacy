import 'package:bloom/ui/calendar/blocs/calendar.dart';
import 'package:bloom/core/calendar/models.dart';
import 'package:flutter/material.dart';
import 'package:table_calendar/table_calendar.dart';
import 'edit_event.dart';
import 'event.dart';

class CalendarView extends StatefulWidget {
  const CalendarView();

  @override
  _CalendarState createState() => _CalendarState();
}

class _CalendarState extends State<CalendarView> with TickerProviderStateMixin {
  CalendarController _calendarController;
  AnimationController _animationController;
  CalendarBloc _bloc;

  @override
  void initState() {
    _bloc = CalendarBloc();

    _calendarController = CalendarController();
    _animationController = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 400),
    );

    _animationController.forward();

    super.initState();
  }

  @override
  void dispose() {
    _bloc.dispose();
    _animationController.dispose();
    _calendarController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    _bloc.findEvents();
    return Scaffold(
      appBar: AppBar(
        title: const Text('Calendar'),
      ),
      body: StreamBuilder<Map<DateTime, List<Event>>>(
        stream: _bloc.eventStream,
        builder: (BuildContext context,
            AsyncSnapshot<Map<DateTime, List<Event>>> snapshot) {
          if (snapshot.hasData) {
            return _buildBody(context, snapshot.data);
          } else {
            return const Center(child: CircularProgressIndicator());
          }
        },
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: () => _newEventPressed(context),
        child: Icon(Icons.add),
        backgroundColor: Colors.red,
      ),
    );
  }

  Widget _buildBody(BuildContext context, Map<DateTime, List<Event>> events) {
    return Column(
      mainAxisSize: MainAxisSize.max,
      children: <Widget>[
        _buildTableCalendar(context, events),
        const SizedBox(height: 8.0),
        Expanded(
          child: StreamBuilder<List<Event>>(
            stream: _bloc.selectedEventsStream,
            initialData: _bloc.selectedEvents,
            builder:
                (BuildContext context, AsyncSnapshot<List<Event>> snapshot) {
              debugPrint('EVENTS LSIT STEAM: $snapshot');
              if (snapshot.hasData) {
                return _buildEventList(context, snapshot.data);
              } else {
                return const Center(child: CircularProgressIndicator());
              }
            },
          ),
        ),
      ],
    );
  }

  Future<void> _newEventPressed(BuildContext ctx) async {
    debugPrint('new event pressed');
    Navigator.push<dynamic>(
      context,
      MaterialPageRoute<dynamic>(
        builder: (BuildContext context) => const EditEventView(),
      ),
    );
  }

  Future<void> _eventPressed(BuildContext ctx, Event event) async {
    debugPrint('event pressed');
    final NoteViewResult res = await Navigator.push<dynamic>(
      context,
      MaterialPageRoute<dynamic>(
        builder: (BuildContext context) => EventView(event: event),
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

  Widget _buildTableCalendar(
      BuildContext context, Map<DateTime, List<Event>> events) {
    return TableCalendar(
      calendarController: _calendarController,
      events: events,
      startingDayOfWeek: StartingDayOfWeek.sunday,
      calendarStyle: CalendarStyle(
        selectedColor: Colors.blue[400],
        todayColor: Colors.blue[200],
        markersColor: Colors.blue[800],
        outsideDaysVisible: false,
      ),
      headerStyle: const HeaderStyle(
        formatButtonVisible: false,
      ),
      onDaySelected: _onDaySelected,
      onVisibleDaysChanged: _onVisibleDaysChanged,
      weekendDays: const <int>[],
    );
  }

  Widget _buildEventList(BuildContext ctx, List<Event> selectedEvents) {
    return ListView(
      children: selectedEvents.map((Event event) {
        return Container(
          decoration: BoxDecoration(
            border: Border.all(width: 0.8),
            borderRadius: BorderRadius.circular(6.0),
          ),
          margin: const EdgeInsets.symmetric(horizontal: 8.0, vertical: 4.0),
          child: ListTile(
            title: Text(event.title),
            onTap: () => _eventPressed(context, event),
          ),
        );
      }).toList(),
    );
  }

  void _onDaySelected(DateTime day, List<dynamic> evts) {
    debugPrint('CALLBACK: _onDaySelected');
    _bloc.updateSelectedDay(day);
  }

  void _onVisibleDaysChanged(
      DateTime firstDay, DateTime lastDay, CalendarFormat format) {
    _bloc.updateStartAndEndDates(firstDay, lastDay);
  }
}
