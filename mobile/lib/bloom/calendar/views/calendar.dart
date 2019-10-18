import 'package:bloom/bloom/calendar/widgets/event.dart';
import 'package:flutter/material.dart';
import 'package:table_calendar/table_calendar.dart';

// Example holidays
final Map<DateTime, List<String>> _holidays = <DateTime, List<String>>{
  DateTime(2019, 1, 1): <String>['New Year\'s Day'],
  DateTime(2019, 1, 6): <String>['Epiphany'],
  DateTime(2019, 2, 14): <String>['Valentine\'s Day'],
  DateTime(2019, 4, 21): <String>['Easter Sunday'],
  DateTime(2019, 4, 22): <String>['Easter Monday'],
};

class CalendarView extends StatefulWidget {
  const CalendarView();

  @override
  _CalendarState createState() => _CalendarState();
}

class _CalendarState extends State<CalendarView> with TickerProviderStateMixin {
  CalendarController _calendarController;
  Map<DateTime, List<String>> _events;
  List<dynamic> _selectedEvents;
  AnimationController _animationController;

  @override
  void initState() {
    super.initState();
    final DateTime _selectedDay = DateTime.now();

    _events = <DateTime, List<String>>{
      _selectedDay.subtract(const Duration(days: 30)): <String>[
        'Event A0',
        'Event B0',
        'Event C0'
      ],
      _selectedDay.subtract(const Duration(days: 27)): <String>['Event A1'],
      _selectedDay.subtract(const Duration(days: 20)): <String>[
        'Event A2',
        'Event B2',
        'Event C2',
        'Event D2'
      ],
      _selectedDay.subtract(const Duration(days: 16)): <String>[
        'Event A3',
        'Event B3'
      ],
      _selectedDay.subtract(const Duration(days: 10)): <String>[
        'Event A4',
        'Event B4',
        'Event C4'
      ],
      _selectedDay.subtract(const Duration(days: 4)): <String>[
        'Event A5',
        'Event B5',
        'Event C5'
      ],
      _selectedDay.subtract(const Duration(days: 2)): <String>[
        'Event A6',
        'Event B6'
      ],
      _selectedDay: <String>['Event A7', 'Event B7', 'Event C7', 'Event D7'],
      _selectedDay.add(const Duration(days: 1)): <String>[
        'Event A8',
        'Event B8',
        'Event C8',
        'Event D8'
      ],
      _selectedDay.add(const Duration(days: 3)): <String>[
        'Event A9',
        'Event A9',
        'Event B9'
      ],
      _selectedDay.add(const Duration(days: 7)): <String>[
        'Event A10',
        'Event B10',
        'Event C10'
      ],
      _selectedDay.add(const Duration(days: 11)): <String>[
        'Event A11',
        'Event B11'
      ],
      _selectedDay.add(const Duration(days: 17)): <String>[
        'Event A12',
        'Event B12',
        'Event C12',
        'Event D12'
      ],
      _selectedDay.add(const Duration(days: 22)): <String>[
        'Event A13',
        'Event B13'
      ],
      _selectedDay.add(const Duration(days: 26)): <String>[
        'Event A14',
        'Event B14',
        'Event C14'
      ],
    };

    _selectedEvents = _events[_selectedDay] ?? <String>[];
    _calendarController = CalendarController();

    _animationController = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 400),
    );

    _animationController.forward();
  }

  @override
  void dispose() {
    _animationController.dispose();
    _calendarController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Calendar'),
      ),
      body: _buildBody(context),
      floatingActionButton: FloatingActionButton(
        onPressed: () => _newEventPressed(context),
        child: Icon(Icons.add),
        backgroundColor: Colors.red,
      ),
    );
  }

  Widget _buildBody(BuildContext context) {
    return Column(
      mainAxisSize: MainAxisSize.max,
      children: <Widget>[
        _buildTableCalendar(context),
        // _buildTableCalendarWithBuilders(),
        const SizedBox(height: 8.0),
        Expanded(child: _buildEventList()),
      ],
    );
  }

  Future<void> _newEventPressed(BuildContext ctx) async {
    debugPrint('new event pressed');
    final NoteViewResult res = await Navigator.push<dynamic>(
      context,
      MaterialPageRoute<dynamic>(
        builder: (BuildContext context) => const EventView(),
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

  Future<void> _eventPressed(BuildContext ctx) async {
    debugPrint('new event pressed');
    final NoteViewResult res = await Navigator.push<dynamic>(
      context,
      MaterialPageRoute<dynamic>(
        builder: (BuildContext context) => const EventView(),
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

  // Simple TableCalendar configuration (using Styles)
  Widget _buildTableCalendar(BuildContext context) {
    return TableCalendar(
      calendarController: _calendarController,
      events: _events,
      holidays: _holidays,
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

  // // More advanced TableCalendar configuration (using Builders & Styles)
  // Widget _buildTableCalendarWithBuilders() {
  //   return TableCalendar(
  //     // locale: 'pl_PL',
  //     calendarController: _calendarController,
  //     events: _events,
  //     holidays: _holidays,
  //     initialCalendarFormat: CalendarFormat.month,
  //     formatAnimation: FormatAnimation.slide,
  //     startingDayOfWeek: StartingDayOfWeek.sunday,
  //     availableGestures: AvailableGestures.all,
  //     availableCalendarFormats: const {
  //       CalendarFormat.month: '',
  //       CalendarFormat.week: '',
  //     },
  //     calendarStyle: CalendarStyle(
  //       outsideDaysVisible: false,
  //       weekendStyle: TextStyle().copyWith(color: Colors.blue[800]),
  //       holidayStyle: TextStyle().copyWith(color: Colors.blue[800]),
  //     ),
  //     daysOfWeekStyle: DaysOfWeekStyle(
  //       weekendStyle: TextStyle().copyWith(color: Colors.blue[600]),
  //     ),
  //     headerStyle: HeaderStyle(
  //       centerHeaderTitle: true,
  //       formatButtonVisible: false,
  //     ),
  //     builders: CalendarBuilders(
  //       selectedDayBuilder: (context, date, _) {
  //         return FadeTransition(
  //           opacity: Tween(begin: 0.0, end: 1.0).animate(_animationController),
  //           child: Container(
  //             margin: const EdgeInsets.all(4.0),
  //             padding: const EdgeInsets.only(top: 5.0, left: 6.0),
  //             color: Colors.deepOrange[300],
  //             width: 100,
  //             height: 100,
  //             child: Text(
  //               '${date.day}',
  //               style: TextStyle().copyWith(fontSize: 16.0),
  //             ),
  //           ),
  //         );
  //       },
  //       todayDayBuilder: (context, date, _) {
  //         return Container(
  //           margin: const EdgeInsets.all(4.0),
  //           padding: const EdgeInsets.only(top: 5.0, left: 6.0),
  //           color: Colors.amber[400],
  //           width: 100,
  //           height: 100,
  //           child: Text(
  //             '${date.day}',
  //             style: TextStyle().copyWith(fontSize: 16.0),
  //           ),
  //         );
  //       },
  //       markersBuilder: (context, date, events, holidays) {
  //         final children = <Widget>[];

  //         if (events.isNotEmpty) {
  //           children.add(
  //             Positioned(
  //               right: 1,
  //               bottom: 1,
  //               child: _buildEventsMarker(date, events),
  //             ),
  //           );
  //         }

  //         if (holidays.isNotEmpty) {
  //           children.add(
  //             Positioned(
  //               right: -2,
  //               top: -2,
  //               child: _buildHolidaysMarker(),
  //             ),
  //           );
  //         }

  //         return children;
  //       },
  //     ),
  //     onDaySelected: (date, events) {
  //       _onDaySelected(date, events);
  //       _animationController.forward(from: 0.0);
  //     },
  //     onVisibleDaysChanged: _onVisibleDaysChanged,
  //   );
  // }

  // Widget _buildEventsMarker(DateTime date, List events) {
  //   return AnimatedContainer(
  //     duration: const Duration(milliseconds: 300),
  //     decoration: BoxDecoration(
  //       shape: BoxShape.rectangle,
  //       color: _calendarController.isSelected(date)
  //           ? Colors.brown[500]
  //           : _calendarController.isToday(date)
  //               ? Colors.brown[300]
  //               : Colors.blue[400],
  //     ),
  //     width: 16.0,
  //     height: 16.0,
  //     child: Center(
  //       child: Text(
  //         '${events.length}',
  //         style: TextStyle().copyWith(
  //           color: Colors.white,
  //           fontSize: 12.0,
  //         ),
  //       ),
  //     ),
  //   );
  // }

  // Widget _buildHolidaysMarker() {
  //   return Icon(
  //     Icons.add_box,
  //     size: 20.0,
  //     color: Colors.blueGrey[800],
  //   );
  // }

  Widget _buildEventList() {
    return ListView(
      children: _selectedEvents
          .map((dynamic event) => Container(
                decoration: BoxDecoration(
                  border: Border.all(width: 0.8),
                  borderRadius: BorderRadius.circular(12.0),
                ),
                margin:
                    const EdgeInsets.symmetric(horizontal: 8.0, vertical: 4.0),
                child: ListTile(
                  title: Text(event.toString()),
                  onTap: () => _eventPressed(context),
                ),
              ))
          .toList(),
    );
  }

  void _onDaySelected(DateTime day, List<dynamic> events) {
    print('CALLBACK: _onDaySelected');
    setState(() {
      _selectedEvents = events;
    });
  }

  void _onVisibleDaysChanged(
      DateTime first, DateTime last, CalendarFormat format) {
    print('CALLBACK: _onVisibleDaysChanged');
  }
}
