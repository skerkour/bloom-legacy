import 'package:bloom/ui/myaccount/widgets/drawer.dart';
import 'package:flutter/material.dart';

class MyAccountDevicesView extends StatefulWidget {
  const MyAccountDevicesView();

  @override
  _MyAccountDevicesState createState() => _MyAccountDevicesState();
}

class _MyAccountDevicesState extends State<MyAccountDevicesView> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      drawer: const MyAccountDrawer(),
      appBar: AppBar(
        title: const Text('My Account'),
      ),
      body: _buildBody(),
    );
  }

  Widget _buildBody() {
    final List<Device> devices = Device.getDevices();

    return Container(
      child: ListView.builder(
          itemCount: devices.length,
          itemBuilder: (BuildContext context, int index) {
            final Device device = devices[index];

            return Card(
              child: ListTile(
                leading: Icon(Icons.phone_android),
                title: Text(device.name),
                subtitle: Text('${device.createdAt} - ${device.ip}'),
                trailing: Icon(Icons.more_vert),
              ),
            );
          }),
    );
  }
}

class Device {
  Device({this.type, this.name}) {
    actions = 'Active';
    createdAt = DateTime.now().toIso8601String();
    ip = '127.0.0.1';
  }

  String type;
  String ip;
  String actions;
  String createdAt;
  String name;

  static List<Device> getDevices() {
    return <Device>[
      Device(type: 'Mobile', name: 'My Phone'),
    ];
  }
}
