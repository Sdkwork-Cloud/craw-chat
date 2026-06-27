import 'package:flutter_test/flutter_test.dart';
import 'package:sdkwork_im_flutter_mobile/app.dart';

void main() {
  testWidgets('renders IM app sign-in shell', (WidgetTester tester) async {
    await tester.pumpWidget(const ImApp());
    await tester.pumpAndSettle();

    expect(find.text('IM App Sign In'), findsOneWidget);
    expect(find.text('Continue with Appbase'), findsOneWidget);
  });
}
