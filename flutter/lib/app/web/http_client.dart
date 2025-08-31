import 'dart:async';
import 'dart:typed_data';

import 'package:etotop_client/src/rust/api/core_bridge/core_bridge.dart';
import 'package:http/http.dart' as http;

class HttpClient {
  final http.Client _client;

  HttpClient({http.Client? client}) : _client = client ?? http.Client();

  Future<HttpResponseImpl> handleRequest(HttpRequestArgsImpl args) async {
    final uri = Uri.parse(args.endpoint);

    final request = http.Request(args.method, uri);

    for (final h in args.headers) {
      request.headers[h.$1] = h.$2;
    }

    if (args.body.isNotEmpty) {
      request.bodyBytes = args.body;
    }

    final duration = Duration(seconds: args.timeoutSecs.toInt());

    try {
      final response = await () async {
        final streamedResponse = await _client.send(request);
        return await http.Response.fromStream(streamedResponse);
      }().timeout(duration);

      if (response.statusCode >= 200 && response.statusCode < 300) {
        return HttpResponseImpl(
          status: 0, // Success
          responseCode: response.statusCode,
          payload: response.bodyBytes,
        );
      } else {
        return HttpResponseImpl(
          status: 3, // HttpError
          responseCode: response.statusCode,
          payload: Uint8List(0),
        );
      }
    } on TimeoutException {
      return HttpResponseImpl(
        status: 1, // Timeout
        responseCode: 0,
        payload: Uint8List(0),
      );
    } catch (e) {
      return HttpResponseImpl(
        status: 2, // NetworkError
        responseCode: 0,
        payload: Uint8List(0),
      );
    }
  }

  void dispose() {
    _client.close();
  }
}
