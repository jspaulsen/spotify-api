import { useState } from 'react'
import Paper from '@mui/material/Paper';
import './App.css'
import { Box, Card, CardContent, CardMedia, Container, LinearProgress, Typography } from '@mui/material';
import React from 'react';


// function LinearDeterminate() {
//   const [progress, setProgress] = React.useState(0);

//   React.useEffect(() => {
//     const timer = setInterval(() => {
//       setProgress((oldProgress) => {
//         if (oldProgress === 100) {
//           return 0;
//         }
//         const diff = Math.random() * 10;
//         return Math.min(oldProgress + diff, 100);
//       });
//     }, 500);

//     return () => {
//       clearInterval(timer);
//     };
//   }, []);

//   return (
//     <Box sx={{ width: '100%' }}>
//       <LinearProgress variant="determinate" value={progress} />
//     </Box>
//   );
// }

function QueueCard(props: { title: string, artist: string }) {
  return (
    <Card sx={{ display: 'flex', backgroundColor: 'lightgray', padding: '0.13rem', 'margin': '0.25rem' }}>
      <Box sx={{ display: 'flex', flexDirection: 'column' }}>
        <Typography variant="caption" sx={{ fontStyle: 'bold' }}>
          {props.title}
        </Typography>
        <Typography
          variant="caption"
          sx={{ color: 'text.secondary' }}
        >
          {props.artist}
        </Typography>
      </Box>
    </Card>
  )
}

function NowPlaying() { 
  return (
    <Card sx={{ display: 'flex', backgroundColor: 'yellow', padding: '0.1rem' }}>
      <Box sx={{ display: 'flex', flexDirection: 'column' }}>
        {/* Small Title */}
        <Typography variant="subtitle1" component="div" sx={{paddingLeft: '0.1rem'}}>
          Now Playing
        </Typography>
        <CardMedia
          component="img"
          sx={{ width: 150 }}
          image="https://cdn.7tv.app/emote/01G6RZ66D0000C5DHX3RQKTNMS/4x.avif"
        />
      </Box>

      <Box sx={{ display: 'flex', flexDirection: 'column' }}>
        <CardContent sx={{ flex: '1 0 auto' }}>
          <Typography component="div" variant="h5">
            Git From My Ass
          </Typography>
          <Typography
            variant="subtitle2"
            component="div"
            sx={{ color: 'text.secondary' }}
          >
            Artist
          </Typography>
          {/* Album */}
          <Typography
            variant="subtitle2"
            component="div"
            sx={{ color: 'text.secondary' }}
          >
            Album
          </Typography>
        </CardContent>
        <Box sx={{ display: 'flex', alignItems: 'center', pl: 1, pb: 1 }}>
          3:23 / 5:00
          {/* <LinearDeterminate /> */}
        </Box>
      </Box>
    </Card>
  );
}

function TitleBar() {
  return (
    <Box>
      <Card sx={{ backgroundColor: 'lightblue', paddingLeft: '0.5rem' }}>
        <Typography variant="h4" component="h1">
          Cannibal's Queue
        </Typography>
      </Card>
    </Box>
  )
}


function App() {
  return (
    <Box paddingLeft='2.5rem' paddingRight='2.5rem' sx={{ display: 'flex', 'height': '97vh', flexDirection: 'column'}}>
      <Paper sx={{ height: '100%', backgroundColor: 'pink' }}>
        <TitleBar />

        <Container maxWidth='sm' sx={{ 'paddingTop': '0.25rem' }}>
          <NowPlaying />
        </Container>

        <Box>
          <Typography variant="h6" component="div" sx={{ paddingLeft: '0.5rem' }}>
            Up Next
          </Typography>
        </Box>

        <Box sx={{ flex: 1, overflow: 'auto', maxHeight: '45%' }}>
          <QueueCard title="Peepee" artist="Poo" /> 
          <QueueCard title="Peepee" artist="Poo" /> 
          <QueueCard title="Peepee" artist="Poo" /> 
          <QueueCard title="Peepee" artist="Poo" /> 
          <QueueCard title="Peepee" artist="Poo" /> 
          <QueueCard title="Peepee" artist="Poo" /> 
          <QueueCard title="Peepee" artist="Poo" /> 
        </Box>
      </Paper>
    </Box>
  )
}

export default App;
