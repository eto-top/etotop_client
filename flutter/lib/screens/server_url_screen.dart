import 'package:flutter/material.dart';
import 'package:etotop_client/app/widgets/buttons/animated_loading_button.dart';
import 'package:etotop_client/app/widgets/buttons/theme_toggler.dart';
import 'package:etotop_client/app/di/get_it.dart';
import 'package:etotop_client/src/rust/api/auth/auth_api.dart';

class ServerUrlScreen extends StatefulWidget {
  const ServerUrlScreen({super.key});

  @override
  // ignore: library_private_types_in_public_api
  _ServerUrlScreenState createState() => _ServerUrlScreenState();
}

class _ServerUrlScreenState extends State<ServerUrlScreen> {
  final _formKey = GlobalKey<FormState>();
  final _urlController = TextEditingController();
  bool _isLoading = false;
  bool _isButtonEnabled = false;
  final _authApi = getIt<AuthApi>();

  @override
  void initState() {
    super.initState();
    _urlController.addListener(_validateUrlOnInput);
    _validateUrlOnInput();
  }

  @override
  void dispose() {
    _urlController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: SafeArea(
        child: SingleChildScrollView(
          padding: const EdgeInsets.all(50.0),
          child: Center(
            child: ConstrainedBox(
              constraints: const BoxConstraints(maxWidth: 400),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.stretch,
                children: [
                  const SizedBox(height: 20),
                  ThemeToggler(),
                  const SizedBox(height: 20),
                  _buildHeader(),
                  const SizedBox(height: 20),
                  _buildEnterUrlForm(),
                  const SizedBox(height: 20),
                  _buildNextButton(),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }

  bool _isValidUrl(String value) {
    final trimmed = value.trim();
    if (trimmed.isEmpty) return false;

    try {
      final uri = Uri.parse(trimmed);
      if (uri.scheme.isEmpty || !{'http', 'https'}.contains(uri.scheme)) {
        return false;
      }
      return true;
    } on FormatException {
      return false;
    }
  }

  void _validateUrlOnInput() {
    final isValid = _isValidUrl(_urlController.text);

    if (isValid != _isButtonEnabled) {
      setState(() {
        _isButtonEnabled = isValid;
      });
    }
  }

  Widget _buildHeader() {
    return Column(
      children: const [
        Text(
          'Enter server url',
          style: TextStyle(fontSize: 32, fontWeight: FontWeight.bold),
        ),
      ],
    );
  }

  Widget _buildEnterUrlForm() {
    return Form(
      key: _formKey,
      child: Column(
        children: [
          // Email Field
          TextFormField(
            controller: _urlController,
            keyboardType: TextInputType.url,
            decoration: InputDecoration(
              labelText: 'Url',
              hintText: 'Enter server url',
              floatingLabelBehavior: FloatingLabelBehavior.always,
              prefixIcon: const Icon(Icons.public),
            ),
            autovalidateMode: AutovalidateMode.onUserInteraction,
            validator: (value) {
              if (value == null || value.trim().isEmpty) {
                return null;
              }
              if (!_isValidUrl(value)) {
                return 'Please enter a valid server url';
              }

              return null;
            },
          ),
          const SizedBox(height: 16),
          const SizedBox(height: 12),
        ],
      ),
    );
  }

  Widget _buildNextButton() {
    return AnimatedLoadingButton(
      text: 'Next',
      canPress: _isButtonEnabled,
      isLoading: _isLoading,
      onPressed: _handleUrl,
    );
  }

  Future<void> _handleUrl() async {
    if (_formKey.currentState!.validate()) {
      setState(() {
        _isLoading = true;
      });

      try {
        _authApi.setUrl(url: _urlController.text.trim());
      } catch (e) {
        if (mounted) {
          ScaffoldMessenger.of(
            context,
          ).showSnackBar(SnackBar(content: Text('Error: ${e.toString()}')));
        }
      } finally {
        if (mounted) {
          setState(() {
            _isLoading = false;
          });
        }
      }
    }
  }
}
