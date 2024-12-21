
import 'package:google_fonts/google_fonts.dart';
import 'package:flutter_highlighting/flutter_highlighting.dart';
import 'package:flutter_highlighting/themes/github-dark.dart';
import 'package:flutter/material.dart';
import 'package:flutter_markdown/flutter_markdown.dart';
import 'package:flutter_svg/flutter_svg.dart';
import 'package:grouped_list/grouped_list.dart';
import 'package:markdown/markdown.dart' as md;
import 'package:hexcolor/hexcolor.dart';
import 'package:flutter_markdown_latex/flutter_markdown_latex.dart';
import 'package:merge_ai/src/rust/api/http_client/blockchain/client.dart';
import 'package:merge_ai/src/rust/api/http_client/open_ai/client.dart';
import 'chat_tools.dart';

class Chat extends StatefulWidget {
  const Chat({super.key});

  @override
  State<Chat> createState() => _ChatState();
}

class _ChatState extends State<Chat> {
  final TextEditingController _controller = TextEditingController();
  String chatLog = "";
  bool auth = false;
  String privKey = "";

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: _buildAppBar(),
      body: Column(
        children: [
          Expanded(
            child: Container(
              decoration: BoxDecoration(
                color: HexColor('000000'),
              ),
              child: GroupedListView<Message, DateTime>(
                padding: const EdgeInsets.all(10),
                elements: messages,
                groupBy: (message) => DateTime(2024),
                groupHeaderBuilder: (Message message) => const SizedBox(),
                itemBuilder: (context, Message message) => _buildMessageCard(message),
              ),
            ),
          ),
          _buildInputField(),
        ],
      ),
    );
  }

  AppBar _buildAppBar() {
    return AppBar(
      title: const Text(
        "Merge AI",
        style: TextStyle(
          color: Colors.amberAccent,
          fontSize: 18,
          fontWeight: FontWeight.w800,
        ),
      ),
      elevation: 0,
      backgroundColor: HexColor('#000000'),
      leading: GestureDetector(
        onTap: () {},
        child: Container(
          margin: const EdgeInsets.all(10),
          alignment: Alignment.center,
          child: SvgPicture.asset('assets/icons/stack.svg'),
          decoration: BoxDecoration(
            color: Colors.amberAccent,
            borderRadius: BorderRadius.circular(100),
          ),
        ),
      ),
    );
  }

  Widget _buildMessageCard(Message message) {
    return Align(
      alignment: message.isSentByMe ? Alignment.centerRight : Alignment.centerLeft,
      child: Card(
        color: HexColor('#00000'),
        margin: message.isSentByMe
            ? const EdgeInsets.only(left: 42, top: 10, bottom: 10, right: 10)
            : const EdgeInsets.only(right: 20, top: 10, bottom: 10, left: 10),
        elevation: 10,
        child: Container(
          decoration: BoxDecoration(
            border: message.isSentByMe
                ? const Border(right: BorderSide(color: Colors.blueAccent, width: 1))
                : const Border(left: BorderSide(color: Colors.grey, width: 1)),
          ),
          padding: const EdgeInsets.all(13),
          child: MarkdownBody(
            selectable: true,
            styleSheet: _markdownStyle(message),
            data: message.text,
            builders: {
              'latex': LatexElementBuilder(
                textStyle: const TextStyle(color: Colors.blueAccent),
              ),
              'code': CodeElementBuilder(),
            },
            extensionSet: _markdownExtensionSet(),
          ),
        ),
      ),
    );
  }

  MarkdownStyleSheet _markdownStyle(Message message) {
    return MarkdownStyleSheet.fromTheme(
      ThemeData(
        textTheme: TextTheme(
          bodyMedium: TextStyle(
            fontSize: 14,
            fontWeight: FontWeight.w200,
            color: HexColor('#E4E0E1'),
          ),
        ),
      ),
    ).copyWith(
      codeblockDecoration: BoxDecoration(
        border: Border.all(color: Colors.white24, width: 0.5),
        borderRadius: BorderRadius.circular(10),
      ),
      h1: TextStyle(color: HexColor('#EEEEEE')),
      h2: TextStyle(color: HexColor('#EEEEEE')),
      h3: TextStyle(color: HexColor('#EEEEEE')),
    );
  }

  md.ExtensionSet _markdownExtensionSet() {
    return md.ExtensionSet(
      [...[LatexBlockSyntax()], ...md.ExtensionSet.gitHubFlavored.blockSyntaxes],
      [...[md.EmojiSyntax(), ...md.ExtensionSet.gitHubFlavored.inlineSyntaxes]],
    );
  }

  Widget _buildInputField() {
    return Container(
      color: HexColor('#000000'),
      padding: const EdgeInsets.symmetric(vertical: 20, horizontal: 20),
      child: TextField(
        controller: _controller,
        style: const TextStyle(fontSize: 14, color: Colors.black),
        decoration: InputDecoration(
          filled: true,
          fillColor: HexColor('#EEEEEE'),
          border: OutlineInputBorder(borderRadius: BorderRadius.circular(14)),
          hintText: "Hva tenker du på?",
        ),
        keyboardType: TextInputType.multiline,
        minLines: 1,
        maxLines: 5,
        textInputAction: TextInputAction.done,
        onSubmitted: (text) => _handleSubmit(text),
      ),
    );
  }

  Future<void> _handleSubmit(String text) async {
    _controller.clear();

    if (!auth) {
      bool isMember = await checkKey(key: text);
      if (isMember) {
        privKey = text;
        auth = true;
        setState(() {
          messages.add(const Message(
            text: "Access Approved! \n\n Welcome!",
            isSentByMe: false,
          ));
        });
      } else {
        setState(() {
          messages.add(const Message(
            text: "Access denied",
            isSentByMe: false,
          ));
        });
      }
    } else {
      chatHistories.add(("user", text));
      String response = await openaiReadResponse(chatLog: chatHistories, key: privKey);
      chatHistories.add(("assistant", response));
      setState(() {
        messages.add(Message(text: text, isSentByMe: true));
        messages.add(Message(text: response, isSentByMe: false));
      });
    }
  }
}

class CodeElementBuilder extends MarkdownElementBuilder {
  @override
  Widget? visitElementAfter(md.Element element, TextStyle? preferredStyle) {
    String language = element.attributes['class']?.substring(9) ?? 'plaintext';
    String content = element.textContent ?? '';

    return SizedBox(
      child: HighlightView(
        content,
        languageId: language,
        theme: githubDarkTheme,
        padding: const EdgeInsets.all(8),
        textStyle: GoogleFonts.robotoMono().copyWith(fontSize: 12),
      ),
    );
  }
}

