import './App.css';
import ReactGA from 'react-ga4';
import Home from "./components/Home"; 
import About from "./components/About";  
import Layout from "./components/Layout"; 
import Instructions from "./components/Instructions"; 
import Policies from "./components/Policies"; 
import Results from "./components/Results"; 
import Contact from "./components/Contact"; 
import Result from "./components/Result";
import Citation from "./components/Citation"; 
import NoMatch from "./components/NoMatch"; 
import { Routes, Route } from "react-router-dom";

function App() {
  ReactGA.initialize([{
    trackingId: process.env.REACT_APP_TRACKING_ID,
    //testMode: true
  }]);
 
  // Setting the routes of the Single Page Application
  return (
    <div className="App">
        <Routes>
            <Route path="/" element={<Layout />}>
                <Route index element={<Home />}/>
                <Route path="about" element={<About />} /> 
                <Route path="policies" element={<Policies />} /> 
                <Route path="instructions" element={<Instructions />} /> 
                <Route path="results" element={<Results />} />
                <Route path="result/:hash/:reqid" element={<Result />} />  
                <Route path="contact" element={<Contact />} /> 
                <Route path="citation" element={<Citation />} /> 

                {/* Using path="*"" means "match anything", so this route
                    acts like a catch-all for URLs that we don't have explicit
                    routes for. */}
                <Route path="*" element={<NoMatch />} />
            </Route>
        </Routes> 
    </div>
  );
}

export default App;
