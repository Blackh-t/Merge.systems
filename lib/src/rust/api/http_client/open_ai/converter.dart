// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.6.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<ChatCompletionRequestMessage>>
abstract class ChatCompletionRequestMessage implements RustOpaqueInterface {}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<ChatResults < String >>>
abstract class ChatResultsString implements RustOpaqueInterface {}

/// Conversations History
class ChatLog {
  final List<ChatCompletionRequestMessage> contents;
  final String pubKey;

  const ChatLog({
    required this.contents,
    required this.pubKey,
  });

  /// Converting the Chat Histories from fromtend into a list of tuplets wuth role definition and text.
  /// The result can be used directly in the Chat-service.
  static Future<ChatLog> msgConvertion(
          {required List<(String, String)> chatLog, required String key}) =>
      RustLib.instance.api
          .crateApiHttpClientOpenAiConverterChatLogMsgConvertion(
              chatLog: chatLog, key: key);

  /// Serialize Data into JSON-format
  Future<ChatResultsString> serialize() =>
      RustLib.instance.api.crateApiHttpClientOpenAiConverterChatLogSerialize(
        that: this,
      );

  @override
  int get hashCode => contents.hashCode ^ pubKey.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ChatLog &&
          runtimeType == other.runtimeType &&
          contents == other.contents &&
          pubKey == other.pubKey;
}
