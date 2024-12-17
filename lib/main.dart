import 'package:flutter/material.dart';
import 'package:merge_ai/src/pages/chat.dart';
import 'package:merge_ai/src/rust/frb_generated.dart';
Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({super.key});
  @override
  State<MyApp> createState() => _MyApp();
}

class _MyApp extends State<MyApp> {
  @override
  Widget build(BuildContext context) {
    return  const MaterialApp(
      debugShowCheckedModeBanner: false,
      home: Chat()
    );
  }
}
