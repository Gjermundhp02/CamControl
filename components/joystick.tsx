import React, { useRef, useState } from 'react';
import { View, StyleSheet, PanResponder, Animated, DimensionValue } from 'react-native';

export type JoystickStyle = {
    top?: DimensionValue
    left?: DimensionValue
    right?: DimensionValue
    bottom?: DimensionValue
    background: {
        backgroundColor: string
        borderColor: string
        borderWidth: number
        diameter: number
    },
    stick: {
        backgroundColor: string
        borderColor: string
        borderWidth: number
        diameter: number
    }
}

type JoystickCallback = React.Dispatch<React.SetStateAction<{
    x: number;
    y: number;
}>>

export default function Joystick ({style, onMove}: {style: JoystickStyle, onMove: JoystickCallback}) {
  const pan = useRef(new Animated.ValueXY()).current;

  const dynamicStyles = StyleSheet.create({
    boundary: {
        position: 'absolute',
        top: style.top,
        left: style.left,
        right: style.right,
        bottom: style.bottom,
        width: style.background.diameter,
        height: style.background.diameter,
        borderRadius: style.background.diameter/2,
        backgroundColor: style.background.backgroundColor,
        borderWidth: style.background.borderWidth,
        borderColor: style.background.borderColor,
        justifyContent: 'center',
        alignItems: 'center',
    },
    handle: {
        width: style.stick.diameter,
        height: style.stick.diameter,
        borderRadius: style.stick.diameter/2,
        backgroundColor: style.stick.backgroundColor,
        borderWidth: style.stick.borderWidth,
        borderColor: style.stick.borderColor,
    },
  });

  const panResponder = useRef(
    PanResponder.create({
      onStartShouldSetPanResponder: () => true,
      onPanResponderMove: (event, gestureState) => {
        const { dx, dy } = gestureState;

        // Constrain the joystick handle within the boundary
        const distance = Math.sqrt(dx * dx + dy * dy);
        const boundaryRadius = style.background.diameter/2; // Radius of the joystick boundary
        if (distance <= boundaryRadius) {
          pan.setValue({ x: dx, y: dy });
          if(onMove) {
            onMove({ x: dx, y: dy });
          }
        } else {
          // Normalize the position to the boundary
          const angle = Math.atan2(dy, dx);
          pan.setValue({
            x: Math.cos(angle) * boundaryRadius,
            y: Math.sin(angle) * boundaryRadius,
          });
          if(onMove) {
            onMove({
                x: Math.cos(angle) * boundaryRadius,
                y: Math.sin(angle) * boundaryRadius,
            });
          }
        }
      },
      onPanResponderRelease: () => {
        // Reset the joystick handle to the center
        Animated.spring(pan, {
          toValue: { x: 0, y: 0 },
          useNativeDriver: false,
        }).start();
        if(onMove) {
          onMove({ x: 0, y: 0 });
        }
      },
    })
  ).current;

  return (
      <View style={dynamicStyles.boundary}>
        <Animated.View
          style={[
            dynamicStyles.handle,
            {
              transform: [{ translateX: pan.x }, { translateY: pan.y }],
            },
          ]}
          {...panResponder.panHandlers}
        />
      </View>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    backgroundColor: '#f0f0f0',
  }
});