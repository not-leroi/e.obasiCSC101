--
-- PostgreSQL database dump
--

-- Dumped from database version 16.1
-- Dumped by pg_dump version 16.1

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: customer_table; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.customer_table (
    c_id integer NOT NULL,
    c_name text NOT NULL,
    c_age integer,
    c_email character(50),
    c_mobile text NOT NULL,
    eid integer,
    data_id integer
);


ALTER TABLE public.customer_table OWNER TO postgres;

--
-- Data for Name: customer_table; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.customer_table (c_id, c_name, c_age, c_email, c_mobile, eid, data_id) FROM stdin;
110	Musta Karim	35	m_karim@gmail.com                                 	8055089112	102	5
111	Lilian Jaiya	43	I_jaiye@gmail.com                                 	8055185341	100	3
112	Arthur Musa	50	a_musan@gmail.com                                 	7055282813	107	10
113	Philip Akonjo	41	p_akonjo@gmail.com                                	9052356772	100	2
114	Marylene Mapa	33	m_mapa@gmail.com                                  	8053333551	117	11
115	Oghenero Agor	50	o_agor@gmail.com                                  	7055566774	102	1
117	Okafor Mathias	45	o_mathias@gmail.com                               	8056763367	120	10
118	Samson Adeleke	65	s_adeleke@gmail.com                               	7056774423	117	11
119	Lawal Tamire	35	l_tamire@gmail.com                                	9056774423	107	5
120	James Job	44	j_job@gmail.com                                   	7051232144	120	2
122	Jimila Adeboye	20	j_adegboye@gmail.com                              	805491923	107	5
116	Adams Bree	33	a_bree@gmail.com                                  	8056763367	120	10
121	Matthew Jakande	21	m_jakande@gmail.com                               	8054921923	107	5
\.


--
-- Name: customer_table customer_table_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customer_table
    ADD CONSTRAINT customer_table_pkey PRIMARY KEY (c_id);


--
-- PostgreSQL database dump complete
--

