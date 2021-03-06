import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';

import '../widget/raw_editor.dart';
import '../widget/selection.dart';
import '../rendering/editor.dart';

/* ----------------------- Selection Gesture Detector ----------------------- */

abstract class EditorTextSelectionGestureDetectorBuilderDelegate {
  GlobalKey<EditorState> getEditableTextKey();

  bool getForcePressEnabled();

  bool getSelectionEnabled();
}

class EditorTextSelectionGestureDetectorBuilder {
  EditorTextSelectionGestureDetectorBuilder(this.delegate);

  final EditorTextSelectionGestureDetectorBuilderDelegate delegate;
  bool shouldShowSelectionToolbar = true;

  EditorState? getEditor() {
    return delegate.getEditableTextKey().currentState;
  }

  RenderEditor? getRenderEditor() {
    return getEditor()!.getRenderEditor();
  }

  void onForcePressStart(ForcePressDetails details) {
    assert(delegate.getForcePressEnabled());
    shouldShowSelectionToolbar = true;
    if (delegate.getSelectionEnabled()) {
      getRenderEditor()!.selectWordsInRange(
        details.globalPosition,
        null,
        SelectionChangedCause.forcePress,
      );
    }
  }

  void onForcePressEnd(ForcePressDetails details) {
    assert(delegate.getForcePressEnabled());
    getRenderEditor()!.selectWordsInRange(
      details.globalPosition,
      null,
      SelectionChangedCause.forcePress,
    );
    if (shouldShowSelectionToolbar) {
      getEditor()!.showToolbar();
    }
  }

  void onTapDown(TapDownDetails details) {
    getRenderEditor()!.handleTapDown(details);

    final kind = details.kind;
    shouldShowSelectionToolbar = kind == null || kind == PointerDeviceKind.touch || kind == PointerDeviceKind.stylus;
  }

  void onTapUp(TapUpDetails details) {
    if (delegate.getSelectionEnabled()) {
      getRenderEditor()!.selectWordEdge(SelectionChangedCause.tap);
    }
  }

  void onTapCancel() {}

  void onLongPressStart(LongPressStartDetails details) {
    if (delegate.getSelectionEnabled()) {
      getRenderEditor()!.selectPositionAt(
        details.globalPosition,
        null,
        SelectionChangedCause.longPress,
      );
    }
  }

  void onLongPressMoveUpdate(LongPressMoveUpdateDetails details) {
    if (delegate.getSelectionEnabled()) {
      getRenderEditor()!.selectPositionAt(
        details.globalPosition,
        null,
        SelectionChangedCause.longPress,
      );
    }
  }

  void onLongPressEnd(LongPressEndDetails details) {
    if (shouldShowSelectionToolbar) {
      getEditor()!.showToolbar();
    }
  }

  void onDoubleTapDown(TapDownDetails details) {
    if (delegate.getSelectionEnabled()) {
      getRenderEditor()!.selectWord(SelectionChangedCause.tap);
      if (shouldShowSelectionToolbar) {
        getEditor()!.showToolbar();
      }
    }
  }

  void onDragSelectionStart(DragStartDetails details) {
    getRenderEditor()!.selectPositionAt(
      details.globalPosition,
      null,
      SelectionChangedCause.drag,
    );
  }

  void onDragSelectionUpdate(DragStartDetails startDetails, DragUpdateDetails updateDetails) {
    getRenderEditor()!.selectPositionAt(
      startDetails.globalPosition,
      updateDetails.globalPosition,
      SelectionChangedCause.drag,
    );
  }

  void onDragSelectionEnd(DragEndDetails details) {}

  Widget build(HitTestBehavior behavior, Widget child) {
    return EditorTextSelectionGestureDetector(
      onTapUp: onTapUp,
      onTapDown: onTapDown,
      onTapCancel: onTapCancel,
      onForcePressStart: delegate.getForcePressEnabled() ? onForcePressStart : null,
      onForcePressEnd: delegate.getForcePressEnabled() ? onForcePressEnd : null,
      onLongPressStart: onLongPressStart,
      onLongPressMoveUpdate: onLongPressMoveUpdate,
      onLongPressEnd: onLongPressEnd,
      onDoubleTapDown: onDoubleTapDown,
      onDragSelectionStart: onDragSelectionStart,
      onDragSelectionUpdate: onDragSelectionUpdate,
      onDragSelectionEnd: onDragSelectionEnd,
      behavior: behavior,
      child: child,
    );
  }
}
