import { FC } from "react";

type CheckMarkerProps = { completed: boolean };

const CheckMarker: FC<CheckMarkerProps> = ({ completed }) => {
    return <div style={{
        position: 'absolute',
        top: -15,
        right: -15,
        borderRadius: 50,
        backgroundColor: completed ? 'darkgreen' : 'darkred',
        height: '3rem',
        width: '3rem',
    }}>

    </div>
}

export default CheckMarker;