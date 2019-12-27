import QtQml 2.12
import QtQuick 2.12
import QtQuick.Window 2.12
import QtQuick.Controls 2.12
import Rustic 1.0

ApplicationWindow {
    id: window
    property alias listView: listView
    visible: true;
    title: "Rustic Music Player"

	menuBar: MenuBar {
		Menu {
			title: qsTr("&File")
			Action {
				text: qsTr("&Quit")
				onTriggered: frontend.exit()
			}
		}
	}

	Frontend {
		id: frontend
	}

    ListView {
        id: listView
        x: 0
        width: 256
        spacing: 0
        anchors.bottom: parent.bottom
        anchors.bottomMargin: 0
        anchors.top: parent.top
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
