// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.6.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// These functions are ignored because they are not marked as `pub`: `openai_send_request`

/// Read response from HTTP-server
Future<String> openaiReadResponse({required List<(String, String)> chatLog}) =>
    RustLib.instance.api
        .crateApiHttpClientOpenAiClientOpenaiReadResponse(chatLog: chatLog);
