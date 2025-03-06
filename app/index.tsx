import { Text, View } from "react-native";
import Video from "react-native-video";

export default function Index() {
  return (
    <Video
        source={{uri: "rtsp://10.22.68.166:8554/mypath"}}
    />
  );
}
