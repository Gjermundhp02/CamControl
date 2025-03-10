import { ActivityIndicator, Animated, GestureResponderEvent, PanResponder, StatusBar, StyleSheet, Text, TouchableWithoutFeedback, View } from "react-native";
import Video from 'react-native-video';
import {lockAsync, unlockAsync, OrientationLock} from 'expo-screen-orientation';
import { useEffect, useRef, useState } from "react";
import { setVisibilityAsync } from 'expo-navigation-bar'
import Joystick, { JoystickStyle } from "@components/joystick";


function move(event: GestureResponderEvent) {
    const { locationX, locationY} = event.nativeEvent
    console.log(locationX, locationY)
}


export default function Index() {
    useEffect(() => {
        StatusBar.setHidden(true)
        lockAsync(OrientationLock.LANDSCAPE)
        setVisibilityAsync("hidden")

        return () => {
            StatusBar.setHidden(false)
            unlockAsync()
        }
    }, [])

    const [joystickPosition, setJoystickPosition] = useState({ x: 0, y: 0 });

    useEffect(() => {
        console.log(joystickPosition)
    }, [joystickPosition])

  return (
    <View style={{backgroundColor:"black", height: "100%", width:"100%"}}>
      <ActivityIndicator style={styles.activityIndicator} size={'large'} />
      <Video
        source={{
            //   uri: 'https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4',
            // uri: 'http://sample.vodobox.net/skate_phantom_flex_4k/skate_phantom_flex_4k.m3u8',
            uri: 'rtsp://192.168.0.136:8554/mypath',
            type: 'rtsp',
            metadata: {
                title: 'Custom Title',
                subtitle: 'Custom Subtitle',
                description: 'Custom Description',
                imageUri:
                'https://pbs.twimg.com/profile_images/1498641868397191170/6qW2XkuI_400x400.png',
            },
        }}
        style={[styles.fullScreen]}
        // fullscreen={true}
        controls={false}
        resizeMode="contain"
        onError={err => console.log(err)}
        >

        </Video>
        <Joystick style={dynamicStyles} onMove={setJoystickPosition} />
    </View>
  );
}

const styles = StyleSheet.create({
    fullScreen: {
        position: 'absolute',
        top: 0,
        left: 0,
        bottom: 0,
        right: 0,
    },
    activityIndicator: {
        position: 'absolute',
        top: 0,
        bottom: 0,
        left: 0,
        right: 0,
    }
});

const dynamicStyles: JoystickStyle = {
    bottom: "5%",
    left: "5%",
    background: {
        diameter: 150,
        backgroundColor: '#ffffff4f',
        borderWidth: 2,
        borderColor: '#ffffff4f',
    },
    stick: {
        diameter: 75,
        backgroundColor: '#ffffff4f',
        borderWidth: 2,
        borderColor: '#ffffff4f',
    },
};