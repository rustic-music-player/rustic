import QtQuick 2.6;
import QtQuick.Window 2.0
import QtQuick.Controls 2.3

Window {
    id: window
    property alias listView: listView
    visible: true;
    title: "Rustic Music Player"

    ToolBar {
        id: toolBar
        anchors.right: parent.right
        anchors.rightMargin: 0
        anchors.left: parent.left
        anchors.leftMargin: 0
        anchors.top: parent.top
        anchors.topMargin: 0
    }

    ListView {
        id: listView
        x: 0
        width: 256
        spacing: 0
        anchors.bottom: parent.bottom
        anchors.bottomMargin: 0
        anchors.top: toolBar.bottom
        anchors.topMargin: 0
        model: ListModel {
            ListElement {
                name: "Library"
                header: true
            }
            ListElement {
                name: "Albums"
            }
            ListElement {
                name: "Artists"
            }
            ListElement {
                name: "Tracks"
            }
            ListElement {
                name: "Playlists"
                header: true
            }
            ListElement {
                name: "Explore"
                header: true
            }
        }
        delegate: Item {
            height: header ? 24 : 40

            Rectangle {
                color: 'transparent'
                anchors.fill: parent

                MouseArea {
                    anchors.fill: parent
                    onEntered: {
                        if (!header) {
                            parent.color = 'rgba(0, 0, 0, 0.5)';
                        }
                    }
                    onExited: {
                        if (!header) {
                            parent.color = 'transparent';
                        }
                    }
                }
            }

            Label {
                z: 2
                text: name
                anchors.topMargin: 0
                anchors.top: window.top
                anchors.left: parent.left
                anchors.leftMargin: 8
                font.bold: header
                anchors.verticalCenter: parent.verticalCenter
                opacity: header ? 0.9 : 1
            }
        }
    }
}
