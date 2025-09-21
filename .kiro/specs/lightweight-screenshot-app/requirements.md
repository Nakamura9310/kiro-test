# Requirements Document

## Introduction

軽量なWindows PC用スクリーンショットアプリケーションの開発。ユーザーがショートカットキーでアプリを起動し、画面の任意の範囲を選択してスクリーンショットを取得、簡単な編集を行い、クリップボードまたはローカルファイルに保存できる機能を提供する。

## Requirements

### Requirement 1

**User Story:** As a Windows PC user, I want to capture screenshots using a keyboard shortcut, so that I can quickly capture screen content without navigating through menus.

#### Acceptance Criteria

1. WHEN the user presses a predefined keyboard shortcut THEN the system SHALL launch the screenshot application
2. WHEN the application launches THEN the system SHALL display a transparent overlay covering the entire screen
3. WHEN the overlay is displayed THEN the system SHALL change the cursor to indicate selection mode

### Requirement 2

**User Story:** As a user, I want to select a rectangular area on my screen using mouse drag, so that I can capture only the specific content I need.

#### Acceptance Criteria

1. WHEN the user clicks and drags on the overlay THEN the system SHALL display a selection rectangle
2. WHEN the user is dragging THEN the system SHALL show the current selection area with visual feedback
3. WHEN the user releases the mouse button THEN the system SHALL capture the selected area as an image
4. WHEN the selection is complete THEN the system SHALL close the overlay and open the editing window

### Requirement 3

**User Story:** As a user, I want to see the captured screenshot in an editing window, so that I can review and modify the image before saving.

#### Acceptance Criteria

1. WHEN a screenshot is captured THEN the system SHALL display the image in a dedicated editing window
2. WHEN the editing window opens THEN the system SHALL show the captured image at an appropriate zoom level
3. WHEN the image is displayed THEN the system SHALL provide zoom in/out functionality
4. WHEN the editing window is active THEN the system SHALL provide editing tools in the interface

### Requirement 4

**User Story:** As a user, I want to add red rectangles around important areas in my screenshot, so that I can highlight specific content.

#### Acceptance Criteria

1. WHEN the user selects the rectangle tool THEN the system SHALL allow drawing rectangles on the image
2. WHEN the user clicks and drags on the image THEN the system SHALL draw a red rectangle outline
3. WHEN a rectangle is drawn THEN the system SHALL make it selectable for modification or deletion
4. WHEN multiple rectangles are added THEN the system SHALL maintain all rectangles as separate editable objects

### Requirement 5

**User Story:** As a user, I want to add text annotations to my screenshot, so that I can provide explanations or labels.

#### Acceptance Criteria

1. WHEN the user selects the text tool THEN the system SHALL allow adding text annotations
2. WHEN the user clicks on the image THEN the system SHALL create a text input field at that location
3. WHEN text is entered THEN the system SHALL display the text as an overlay on the image
4. WHEN text is added THEN the system SHALL make it selectable for editing, moving, or deletion

### Requirement 6

**User Story:** As a user, I want to copy the edited screenshot to clipboard, so that I can paste it directly into other applications.

#### Acceptance Criteria

1. WHEN the user clicks the "Copy to Clipboard" button THEN the system SHALL copy the current edited image to the Windows clipboard
2. WHEN the image is copied THEN the system SHALL show a confirmation message
3. WHEN the image is in clipboard THEN the user SHALL be able to paste it in other applications
4. WHEN copying is complete THEN the system SHALL maintain the original image quality

### Requirement 7

**User Story:** As a user, I want to save the edited screenshot as a local file, so that I can store it for future reference.

#### Acceptance Criteria

1. WHEN the user clicks the "Save" button THEN the system SHALL open a file save dialog
2. WHEN saving THEN the system SHALL support common image formats (PNG, JPG, BMP)
3. WHEN a file location is selected THEN the system SHALL save the edited image to the specified location
4. WHEN saving is complete THEN the system SHALL show a confirmation message with the file path

### Requirement 8

**User Story:** As a user, I want the application to be lightweight and responsive, so that it doesn't impact my system performance.

#### Acceptance Criteria

1. WHEN the application starts THEN the system SHALL launch within 2 seconds
2. WHEN capturing screenshots THEN the system SHALL complete the capture within 1 second
3. WHEN the application is idle THEN the system SHALL use minimal system resources
4. WHEN the application closes THEN the system SHALL release all allocated resources properly

### Requirement 9

**User Story:** As a user, I want to cancel the screenshot process, so that I can exit without capturing if I change my mind.

#### Acceptance Criteria

1. WHEN the user presses the Escape key during selection THEN the system SHALL cancel the screenshot process
2. WHEN canceling THEN the system SHALL close the overlay and return to normal desktop
3. WHEN the user closes the editing window without saving THEN the system SHALL prompt for confirmation
4. WHEN confirmed to close THEN the system SHALL discard the current screenshot and close the application