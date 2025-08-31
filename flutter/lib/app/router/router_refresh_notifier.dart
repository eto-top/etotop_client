import 'package:flutter/material.dart';
import 'dart:async';

class RouterRefreshNotifier extends ChangeNotifier {
  late final StreamSubscription _subscription;

  RouterRefreshNotifier(Stream stream) {
    notifyListeners();
    _subscription = stream.listen((_) => notifyListeners());
  }

  @override
  void dispose() {
    _subscription.cancel();
    super.dispose();
  }
}
