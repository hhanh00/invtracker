import 'package:flutter/material.dart';
import 'package:invoice_tracker/src/rust/api/simple.dart';
import 'package:invoice_tracker/src/rust/frb_generated.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<StatefulWidget> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  String msg = "";
  @override
  void initState() {
    super.initState();

    Future(() async {
      msg = await candleTest();
      setState(() {});
    });
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('flutter_rust_bridge quickstart')),
        body: Center(
          child: Text(msg),
        ),
      ),
    );
  }
}
