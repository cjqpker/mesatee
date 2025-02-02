// Copyright 2019 MesaTEE Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#include "lru_cache.h"



lru_cache::lru_cache()
{
	m_it = list.begin();
}


lru_cache::~lru_cache()
{
	list_node_t* list_node;
	map_node_t* map_node;
	
	while (list.size() > 0)
	{
		list_node = list.front();
		map_node = map[list_node->key];
		
		map.erase(list_node->key);
		delete map_node;

		list.pop_front();
		delete list_node;
	}

	assert(list.empty() == true);
	assert(map.empty() == true);
}


void lru_cache::rehash(uint32_t size_)
{
	map.rehash(size_);
}


bool lru_cache::add(uint64_t key, void* data)
{
	map_node_t* map_node = NULL;
	list_node_t* list_node = NULL;
	
	try {
		map_node = new map_node_t();
		list_node = new list_node_t();
	}
	catch (std::bad_alloc& e) {
		(void)e; // remove warning
		return false;
	}
	
	list_node->key = key;
	list.push_front(list_node);

	map_iterator map_it = map.find(key);
	assert(map_it == map.end());
	if (map_it != map.end())
	{
		// this indicates some fatal problem, perhaps race issue caused by bad locks...
		map_node_t* tmp_map_node = map_it->second;
		if (tmp_map_node != NULL)
			delete tmp_map_node;
		map.erase(map_it);
	}

	map_node->data = data;
	map_node->list_it = list.begin();
	
	map[key] = map_node;
	
	return true;
}


void* lru_cache::find(uint64_t key)
{
	map_iterator map_it = map.find(key);
	if (map_it == map.end())
		return NULL;

	map_node_t* map_node = map_it->second;
	return map_node->data;
}


void* lru_cache::get(uint64_t key)
{
	map_iterator map_it = map.find(key);
	if (map_it == map.end())
		return NULL;

	map_node_t* map_node = map_it->second;

	list_node_t* list_node = *(map_node->list_it);
	assert(list_node != NULL);
	if (list_node == NULL) // this should never happen, but just in case, code is here to fix it
	{
		try {
			list_node = new list_node_t();
			list_node->key = map_it->first;
		}
		catch (std::bad_alloc& e) {
			(void)e; // remove warning
			return NULL;
		}
	}
	
	list.erase(map_node->list_it);
	list.push_front(list_node);

	map_node->list_it = list.begin();

	return map_node->data;
}


uint32_t lru_cache::size()
{
	assert(list.size() == map.size());
	return (uint32_t)list.size();
}


void* lru_cache::get_first()
{
	if (list.size() == 0)
		return NULL;

	m_it = list.begin();
	if (m_it == list.end())
		return NULL;

	list_node_t* list_node = (*m_it);
	assert(list_node != NULL);
	if (list_node == NULL)
		return NULL;

	map_iterator map_it = map.find(list_node->key);
	assert(map_it != map.end());
	if (map_it == map.end())
		return NULL;

	map_node_t* map_node = map_it->second;
	assert(map_node != NULL);
	if (map_node == NULL)
		return NULL;

	return map_node->data;
}

void* lru_cache::get_next()
{
	if (list.size() == 0)
		return NULL;

	++m_it;

	if (m_it == list.end())
		return NULL;

	list_node_t* list_node = (*m_it);
	assert(list_node != NULL);
	if (list_node == NULL)
		return NULL;

	map_iterator map_it = map.find(list_node->key);
	assert(map_it != map.end());
	if (map_it == map.end())
		return NULL;

	map_node_t* map_node = map_it->second;
	assert(map_node != NULL);
	if (map_node == NULL)
		return NULL;

	return map_node->data;
}


void* lru_cache::get_last()
{
	if (list.size() == 0)
		return NULL;

	list_iterator it = list.end(); // pointer to the object past-the-end
	assert(it != list.begin());
	if (it == list.begin()) // the list is empty
		return NULL;
	
	--it; // now it points to the last object
	list_node_t* list_node = (*it);
	assert(list_node != NULL);
	if (list_node == NULL)
		return NULL;

	map_iterator map_it = map.find(list_node->key);
	assert(map_it != map.end());
	if (map_it == map.end())
		return NULL;

	map_node_t* map_node = map_it->second;
	assert(map_node != NULL);
	if (map_node == NULL)
		return NULL;

	return map_node->data;
}


void lru_cache::remove_last()
{
	uint64_t key;

	list_iterator it = list.end(); // pointer to the object past-the-end
	if (it == list.begin()) // the list is empty
		return;

	--it; // now it points to the last object

	list_node_t* list_node = (*it);
	list.erase(it);
	
	assert(list_node != NULL);
	if (list_node == NULL) // unclear how this can happen...
		return;

	key = list_node->key;
	delete list_node;

	map_iterator map_it = map.find(key);
	assert(map_it != map.end());
	if (map_it == map.end())
		return;

	map_node_t* map_node = map_it->second;
	assert(map_node != NULL);
	if (map_node == NULL)
		return;

	map.erase(key);
	delete map_node;
}

