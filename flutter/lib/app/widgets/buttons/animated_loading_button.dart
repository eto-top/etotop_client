import 'package:flutter/material.dart';

class AnimatedLoadingButton extends StatelessWidget {
  final String text;
  final bool canPress;
  final bool isLoading;
  final VoidCallback? onPressed;

  const AnimatedLoadingButton({
    super.key,
    required this.text,
    required this.canPress,
    this.isLoading = false,
    this.onPressed,
  });

  @override
  Widget build(BuildContext context) {
    final bool isActionable = canPress && !isLoading;

    return AnimatedOpacity(
      opacity: isActionable ? 1.0 : 0.5,
      duration: const Duration(milliseconds: 300),
      curve: Curves.easeInOut,
      child: ElevatedButton(
        onPressed: isActionable ? onPressed : null,
        child: isLoading
            ? const SizedBox(
                height: 20,
                width: 20,
                child: CircularProgressIndicator(
                  strokeWidth: 2,
                  valueColor: AlwaysStoppedAnimation<Color>(Colors.white),
                ),
              )
            : Text(text, style: const TextStyle(fontSize: 16)),
      ),
    );
  }
}
