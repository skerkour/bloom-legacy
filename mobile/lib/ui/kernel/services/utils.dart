import 'package:intl/intl.dart';

class Utils {
  static String stringForDatetime(DateTime dt) {
    final DateTime dtInLocal = dt.toLocal();
    //DateTime.fromMillisecondsSinceEpoch( 1490489845  * 1000).toLocal(); //year:  1490489845 //>day: 1556152819  //month:  1553561845  //<day: 1556174419
    final DateTime now = DateTime.now().toLocal();
    String dateString = 'Edited ';

    final Duration diff = now.difference(dtInLocal);

    if (now.day == dtInLocal.day) {
      // creates format like: 12:35 PM,
      final DateFormat todayFormat = DateFormat('h:mm a');
      dateString += todayFormat.format(dtInLocal);
    } else if ((diff.inDays) == 1 ||
        (diff.inSeconds < 86400 && now.day != dtInLocal.day)) {
      final DateFormat yesterdayFormat = DateFormat('h:mm a');
      dateString += 'Yesterday, ' + yesterdayFormat.format(dtInLocal);
    } else if (now.year == dtInLocal.year && diff.inDays > 1) {
      final DateFormat monthFormat = DateFormat('MMM d');
      dateString += monthFormat.format(dtInLocal);
    } else {
      final DateFormat yearFormat = DateFormat('MMM d y');
      dateString += yearFormat.format(dtInLocal);
    }

    return dateString;
  }

  static DateTime fromGoTime(String time) {
    if (time == null) {
      return null;
    }

    if (time.length > 25) {
      time = time.substring(0, 25);
    }

    time = time.replaceAll('Z', '') + 'Z';

    return DateTime.parse(time);
  }
}
