import { makeStyles, createStyles } from "@mui/material/styles";

export const notificationsStyles = makeStyles((theme: any) =>
    createStyles({
        root: {
            maxWidth: 345,
            postion: "fixed",
        },
        button: {
            border: "2px solid white",
            backgroundColor: theme.palette.background.default,
        },
    })
);
