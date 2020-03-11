import 'package:bloom/ui/myaccount/widgets/drawer.dart';
import 'package:flutter/material.dart';

class MyAccountBillingView extends StatefulWidget {
  const MyAccountBillingView();

  @override
  _MyAccountBillingState createState() => _MyAccountBillingState();
}

class _MyAccountBillingState extends State<MyAccountBillingView> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      drawer: const MyAccountDrawer(),
      appBar: AppBar(
        title: const Text('MyAccount'),
      ),
      body: _buildBody(),
    );
  }

  Widget _buildBody() {
    return Container(
      padding: const EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: <Widget>[
          const Text('Payment methods', style: TextStyle(fontSize: 32)),
          _buildPaymentMethodsTable(),
          const Divider(),
          const Text('Subscriptions', style: TextStyle(fontSize: 32)),
          _buildSubscriptionsTable(),
          const Divider(),
          const Text('Invoices', style: TextStyle(fontSize: 32)),
          _buildInvoicesTable(),
        ],
      ),
    );
  }

  Widget _buildPaymentMethodsTable() {
    final List<PaymenMethod> methods = PaymenMethod.getMethods();
    final List<DataRow> rows = methods
        .map(
          (PaymenMethod method) => DataRow(cells: <DataCell>[
            DataCell(
              Text(method.type),
            ),
          ]),
        )
        .toList();

    final List<DataColumn> columns = <DataColumn>[
      const DataColumn(
        label: Text('Type'),
        numeric: false,
      ),
    ];

    return SingleChildScrollView(
      scrollDirection: Axis.vertical,
      child: SingleChildScrollView(
        scrollDirection: Axis.horizontal,
        child: DataTable(
          rows: rows,
          columns: columns,
        ),
      ),
    );
  }

  Widget _buildSubscriptionsTable() {
    final List<Subscription> methods = Subscription.getSubscriptions();
    final List<DataRow> rows = methods
        .map(
          (Subscription method) => DataRow(cells: <DataCell>[
            DataCell(
              Text(method.type),
            ),
          ]),
        )
        .toList();

    final List<DataColumn> columns = <DataColumn>[
      const DataColumn(
        label: Text('Type'),
        numeric: false,
      ),
    ];

    return SingleChildScrollView(
      scrollDirection: Axis.vertical,
      child: SingleChildScrollView(
        scrollDirection: Axis.horizontal,
        child: DataTable(
          rows: rows,
          columns: columns,
        ),
      ),
    );
  }

  Widget _buildInvoicesTable() {
    final List<Invoice> methods = Invoice.getInvoices();
    final List<DataRow> rows = methods
        .map(
          (Invoice method) => DataRow(cells: <DataCell>[
            DataCell(
              Text(method.type),
            ),
          ]),
        )
        .toList();

    final List<DataColumn> columns = <DataColumn>[
      const DataColumn(
        label: Text('Type'),
        numeric: false,
      ),
    ];

    return SingleChildScrollView(
      scrollDirection: Axis.vertical,
      child: SingleChildScrollView(
        scrollDirection: Axis.horizontal,
        child: DataTable(
          rows: rows,
          columns: columns,
        ),
      ),
    );
  }
}

class PaymenMethod {
  PaymenMethod({this.type});

  String type;

  static List<PaymenMethod> getMethods() {
    return <PaymenMethod>[
      PaymenMethod(type: 'card'),
    ];
  }
}

class Subscription {
  Subscription({this.type});

  String type;

  static List<Subscription> getSubscriptions() {
    return <Subscription>[
      Subscription(type: 'card'),
    ];
  }
}

class Invoice {
  Invoice({this.type});

  String type;

  static List<Invoice> getInvoices() {
    return <Invoice>[
      Invoice(type: 'card'),
    ];
  }
}
